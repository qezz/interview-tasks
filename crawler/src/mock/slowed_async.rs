use async_trait::async_trait;

use std::{
    collections::HashMap,
    time::Duration
};

use crate::common::*;

#[derive(Clone, Debug)]
pub struct TokioSlowedClient<T> {
    sleep_duration: Duration,
    inner: HashMap<String, T>,
}

impl<T> TokioSlowedClient<T> {
    pub(crate) fn new(data: HashMap<String, T>, dur: Duration) -> Self {
        Self {
            sleep_duration: dur,
            inner: data,
        }
    }
}

#[async_trait]
impl<T: Clone + Send + Sync, E: From<ClientError>> AsyncClientInterface<T, E> for TokioSlowedClient<T> {
    async fn get(&self, url: &str) -> Result<T, E> {
        let value = self.inner.get(url).ok_or(ClientError::Fake)?;
        tokio::time::sleep(self.sleep_duration).await;
        Ok((*value).clone())
    }
}
