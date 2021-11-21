use std::{collections::HashMap, time::Duration, thread};

use log::trace;

use crate::common::*;

#[derive(Clone, Debug)]
pub (crate) struct SlowedClient<T> {
    sleep_duration: Duration,
    inner: HashMap<String, T>,
}

impl<T> SlowedClient<T> {
    pub(crate) fn new(data: HashMap<String, T>, dur: Duration) -> Self {
        Self {
            sleep_duration: dur,
            inner: data,
        }
    }
}

impl<T: Clone + Send + Sync, E: From<ClientError>> ClientInterface<T, E> for SlowedClient<T> {
    fn get(&self, url: &str) -> Result<T, E> {
        trace!("[{:?}] FETCHING {}", thread::current().id(), url);
        let value = self.inner.get(url).ok_or(ClientError::Fake)?;
        trace!("[{:?}] SUCCESS (SLEEP) {}", thread::current().id(), url);
        thread::sleep(self.sleep_duration);
        Ok((*value).clone())
    }
}
