use crate::common::*;

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::thread;

use::log::trace;

type Address = String;

#[derive(Debug, Clone)]
pub enum NodeStatus {
    Success,
    Failure,
}

type ClientType = Arc<Mutex<Box<dyn ClientInterface<Vec<String>, ClientError>>>>;
type Registry = Arc<Mutex<HashMap<Address, NodeStatus>>>;

pub fn discover(url: Address, client: ClientType) -> Result<Vec<String>, ClientError> {
    let c = client.lock().unwrap();
    c.get(&url)
}

pub fn crawl(reg: Registry, base_url: Address, client: ClientType, should_stop: Arc<Mutex<bool>>) {
    trace!("[{:?}] CRAWL FOR {}", thread::current().id(), base_url);
    let mut handles = Vec::new(); // Could be replaced with `::with_capacity(...)`

    // Check if still active
    if *(should_stop.lock().unwrap()) {
        trace!("[{:?}] TIMER EXCEEDED AT {}", thread::current().id(), base_url);
        return;
    }

    // Fetch list of nodes
    let res = discover(base_url.clone(), client.clone());
    let urls_res = match res {
        Ok(urls) => {
            (urls, NodeStatus::Success)
        },
        Err(_) => {
            (vec![], NodeStatus::Failure)
        }
    };

    // Add record to registry
    {
        reg.lock().unwrap().insert(base_url, urls_res.1);
    }

    trace!("[{:?}] NEXT: {:?}", thread::current().id(), urls_res.0);

    for url in urls_res.0 {
        let rg = reg.lock().unwrap();

        match rg.get(&url) {
            Some(_) => {
                // already discovered
                continue
            },
            None => {
                // Clone Arc Mutexes to be shared across threads
                let _cloned_client = client.clone();
                let _cloned_token = should_stop.clone();
                let _cloned_reg = reg.clone();

                let h = thread::spawn(move || {
                    crawl(_cloned_reg, url, _cloned_client, _cloned_token)
                });

                handles.push(h);
            }
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use std::time::Duration;

    use crate::mock::SlowedClient;

    #[test]
    fn bbb() {
        let fake_data: HashMap<String, Vec<String>> = vec![
            ("1".into(), vec!["2".into(), "3".into()]),
            ("2".into(), vec!["5".into(), "1".into()]),
        ].into_iter().collect();

        let reg = Arc::new(Mutex::new(HashMap::new()));
        let should_stop = Arc::new(Mutex::new(false));

        let _ = crawl(reg.clone(),
                        "1".into(),
                        Arc::new(Mutex::new(Box::new(fake_data))),
                        should_stop,
        );

        println!("{:?}", reg.lock().unwrap());
    }

    #[test]
    fn slowed() {
        let fake_data: HashMap<String, Vec<String>> = vec![
            ("1".into(), vec!["2".into(), "3".into()]),
            ("2".into(), vec!["5".into(), "1".into()]),
            ("5".into(), vec!["1".into(), "7".into(), "20".into()]),
            ("7".into(), vec!["8".into(), "9".into()]),
            ("8".into(), vec!["11".into(), "12".into()]),
            ("9".into(), vec!["13".into(), "14".into()]),
            ("20".into(), vec!["21".into(), "22".into()]),
        ].into_iter().collect();

        let client =
            SlowedClient::new(fake_data, Duration::from_secs(1));

        let reg = Arc::new(Mutex::new(HashMap::new()));
        let should_stop = Arc::new(Mutex::new(false));

        // Timeout
        let _cloned_stop = should_stop.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(4600));
            *(_cloned_stop.lock().unwrap()) = true;
        });

        let _ = crawl(reg.clone(),
                        "1".into(),
                        Arc::new(Mutex::new(Box::new(client))),
                        should_stop.clone(),
        );

        println!("{:?}", reg.lock().unwrap());
        assert!(
            *(should_stop.lock().unwrap())
        )
    }
}
