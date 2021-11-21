use crate::common::*;

use std::{collections::HashMap, sync::Arc};

use tokio::sync::{Mutex, oneshot};
use deadqueue::unlimited::Queue;
use tracing::{debug, trace};

type Address = String;

#[derive(Debug, Clone)]
pub enum NodeStatus {
    Success,
    Failure,
}

type ClientType<C> = Arc<Mutex<C>>; //  where C: TokioSlowedClient<String, Vec<String>>;

pub async fn discover<C>(url: Address, client: ClientType<C>) -> Result<Vec<String>, ClientError>
where C: AsyncClientInterface<Vec<String>, ClientError>
{
    let c = &*client.lock().await;
    c.get(&url).await
}

pub enum QueueItem<T> {
    Value(T),
    Stop,
}

pub async fn do_crawl_and_return<C>(init: String, client: C, stop_one_shot: oneshot::Receiver<()>) -> HashMap<String, NodeStatus>
where C: AsyncClientInterface<Vec<String>, ClientError>
{
    let reg = Arc::new(Mutex::new(HashMap::new()));

    do_crawl_modify(reg.clone(), init, client, stop_one_shot).await;

    let mutex = Arc::try_unwrap(reg).expect("cannot acquire lock");
    mutex.into_inner()
}

pub async fn do_crawl_modify<C>(reg: Arc<Mutex<HashMap<String, NodeStatus>>>, init: String, client: C, stop_one_shot: oneshot::Receiver<()>)
where C: AsyncClientInterface<Vec<String>, ClientError>
{
    debug!("Start");
    let q = {
        let inner_q: Queue<QueueItem<String>> = Queue::new();
        Arc::new(inner_q)
    };

    debug!("Spawning delayed stop receiver");
    let _q = q.clone();
    tokio::spawn(async move {
        match stop_one_shot.await {
            Ok(_) => _q.push(QueueItem::Stop),
            Err(_) => debug!("the sender dropped"),
        }
    });

    let wrapped_client = Arc::new(Mutex::new(client));
    // let reg = Arc::new(Mutex::new(registry));

    debug!("Pushing init value");
    q.push(QueueItem::Value(init));

    loop {
        trace!("Waiting for new task");
        let item = q.pop().await;
        match item {
            QueueItem::Value(val) => {
                let already_present = {
                    let reg = reg.lock().await;
                    reg.contains_key(&val)
                };

                if already_present {
                    continue
                }

                let _wc = wrapped_client.clone();
                let d = discover(val.clone(), _wc).await;
                let (urls_, node_status) = match d {
                    Ok(urls) => {
                        (urls, NodeStatus::Success)
                    }
                    Err(_) => {
                        (vec![], NodeStatus::Failure)
                    }
                };

                {
                    let mut reg = reg.lock().await;
                    reg.insert(val, node_status);
                }

                for it in urls_ {
                    q.push(QueueItem::Value(it));
                }
            },
            QueueItem::Stop => {
                trace!("Stop received");
                break
            }
        }

        trace!("Registry state {:?}", reg.lock().await);
    }

    trace!("Stopping");
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::TokioSlowedClient;

    use std::time::Duration;

    use test_log::test;
    // use tracing_subscriber;

    fn to_hm(vec: Vec<(&str, &[&str])>) -> HashMap<String, Vec<String>> {
        vec
            .into_iter()
            .map(|(k, vals)| {
                (
                    k.to_string(),
                    vals.to_vec()
                        .iter()
                        .map(|n| n.to_string())
                        .collect()
                )
            })
            .collect()
    }

    fn fake_data() -> HashMap<String, Vec<String>> {
        let v: Vec<(&str, &[&str])> = vec![
            ("1", &["2", "3"]),
            ("2", &["5", "1"]),
            ("5", &["1", "7", "20"]),
            ("7", &["8", "9"]),
            ("8", &["11", "12"]),
            ("9", &["13", "14"]),
            ("20", &["21", "22"]),
        ];

        to_hm(v)
    }

    #[test(tokio::test)]
    async fn tokio_timeout() {
        let _ = env_logger::builder().is_test(true).try_init();

        let client =
            TokioSlowedClient::new(fake_data(), Duration::from_secs(1));
        let (_, rx) = oneshot::channel();

        let res = do_crawl_and_return("1".into(), client, rx);

        let timeout = Duration::from_secs(6);
        if let Err(_) = tokio::time::timeout(timeout, res).await {
            println!("interrupted after {:?}", timeout);
        }
    }

    #[test(tokio::test)]
    async fn manual_stop() {
        let _ = env_logger::builder().is_test(true).try_init();

        let client =
            TokioSlowedClient::new(fake_data(), Duration::from_secs(1));
        let (tx, rx) = oneshot::channel();

        let res = do_crawl_and_return("1".into(), client, rx);
        let handle = tokio::spawn(res);

        let timeout = Duration::from_secs(6);
        let timeout_handle = tokio::spawn(async move {
            tokio::time::sleep(timeout).await;
            tx.send(()).unwrap();
        });

        handle.await.unwrap();
        timeout_handle.await.unwrap();
    }

    #[test(tokio::test)]
    async fn instant_client() {
        let _ = env_logger::builder().is_test(true).try_init();

        let (_tx, rx) = oneshot::channel();

        let res = do_crawl_and_return("1".into(), fake_data(), rx);
        // let handle = tokio::spawn(res);

        let timeout = Duration::from_secs(6);
        if let Err(_) = tokio::time::timeout(timeout, res).await {
            println!("interrupted after {:?}", timeout);
        }
    }
}
