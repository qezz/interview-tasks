use std::collections::HashMap;

use async_trait::async_trait;

use crate::common::*;

impl<T: Clone + Send + Sync, E: From<ClientError>> ClientInterface<T, E> for HashMap<String, T> {
    fn get(&self, url: &str) -> Result<T, E> {
        let value = self.get(url).ok_or(ClientError::Fake)?;
        Ok((*value).clone())
    }
}

#[async_trait]
impl<T: Clone + Send + Sync, E: From<ClientError>> AsyncClientInterface<T, E> for HashMap<String, T> {
    async fn get(&self, url: &str) -> Result<T, E> {
        let value = self.get(url).ok_or(ClientError::Fake)?;
        Ok((*value).clone())
    }
}
