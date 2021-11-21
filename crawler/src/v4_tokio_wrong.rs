use tokio::sync::{Mutex, oneshot};
use deadqueue::unlimited::Queue;
use tracing::{debug, trace};

use std::{collections::{HashMap, hash_map::Entry}, sync::Arc};

use crate::common::*;

type Address = String;

#[derive(Debug, Clone)]
pub enum NodeStatus {
    Success,
    Failure,
}

type ClientType<C> = Arc<Mutex<C>>;

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

/// This version has some major flaws
///
/// First, it's not returning any value, nor changing one outside
///
/// Second, Mutex is locked for a really long time, which is not good.
pub async fn do_crawl_and_return<C>(init: String, client: C, stop_one_shot: oneshot::Receiver<()>)
where C: AsyncClientInterface<Vec<String>, ClientError>
{
    debug!("Start");
    let q = {
        let inner_q: Queue<QueueItem<String>> = Queue::new();
        Arc::new(inner_q)
    };

    debug!("Spawning delayed stop received");
    let _q = q.clone();
    tokio::spawn(async move {
        match stop_one_shot.await {
            Ok(_) => _q.push(QueueItem::Stop),
            Err(_) => debug!("the sender dropped"),
        }
    });

    let wrapped_client = Arc::new(Mutex::new(client));

    debug!("Pushing init value");
    q.push(QueueItem::Value(init));

    let reg = Arc::new(Mutex::new(HashMap::new()));

    loop {
        trace!("Waiting for new task");
        let item = q.pop().await;
        match item {
            QueueItem::Value(val) => {
                let mut reg = reg.lock().await;
                if let Entry::Vacant(e) = reg.entry(val.clone()) {
                    let _wc = wrapped_client.clone();

                    // FIXME: Generally, it's not good to lock the Mutex for a long time.
                    // Hence, this could really do this. It should be fixed.
                    let d = discover(val, _wc).await; // FIXME
                    let (urls_, res_) = match d {
                        Ok(urls) => {
                            (urls, NodeStatus::Success)
                        }
                        Err(_) => {
                            (vec![], NodeStatus::Failure)
                        }
                    };
                    e.insert(res_);
                    for it in urls_ {
                        q.push(QueueItem::Value(it));
                    }
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
