use async_trait::async_trait;

/// AsyncClientInterface is an async abstraction over HTTP Client (sync)
#[async_trait]
pub trait AsyncClientInterface<T, E>: Send + Sync {
    async fn get(&self, url: &str) -> Result<T, E>;
}
