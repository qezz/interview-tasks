/// ClientInterface is an abstraction over HTTP Client (sync)
pub trait ClientInterface<T, E>: Send + Sync {
    fn get(&self, url: &str) -> Result<T, E>;
}

// impl<T: for <'de> serde::de::Deserialize<'de> + Send + Sync, E: From<reqwest::Error>> ClientInterface<T, E> for Arc<Mutex<reqwest::blocking::Client>> {
//     fn get(&self, url: &str) -> Result<T, E> {
//         let client = (*self).lock().unwrap();
//         let resp = client.get(url).send().map_err(Into::into)?;

//         Ok(resp.json::<T>().unwrap())
//     }
// }
