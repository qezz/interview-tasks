use crate::common::*;

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::thread;

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

pub fn crawl(reg: Registry, base_url: Address, client: ClientType) {
    let mut handles = Vec::new();

    let res = discover(base_url.clone(), client.clone());
    let urls_res = match res {
        Ok(urls) => {
            (urls, NodeStatus::Success)
        },
        Err(_) => {
            (vec![], NodeStatus::Failure)
        }
    };

    {
        reg.lock().unwrap().insert(base_url, urls_res.1);
    }

    for url in urls_res.0 {
        let rg = reg.lock().unwrap();

        match rg.get(&url) {
            Some(_) => {
                // already discovered
                continue
            },
            None => {
                let _cloned_client = client.clone();
                let _cloned_reg = reg.clone();

                let h = thread::spawn(move || {
                    crawl(_cloned_reg, url, _cloned_client)
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

    #[test]
    fn bbb() {
        let fake_data: HashMap<String, Vec<String>> = vec![
            ("1".into(), vec!["2".into(), "3".into()]),
            ("2".into(), vec!["5".into(), "1".into()]),
        ].into_iter().collect();

        let reg = Arc::new(Mutex::new(HashMap::new()));

        let _res = crawl(reg.clone(), "1".into(), Arc::new(Mutex::new(Box::new(fake_data))));

        println!("{:?}", reg.lock().unwrap());
    }
}
