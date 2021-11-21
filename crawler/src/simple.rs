use crate::common::*;

pub struct Crawler;

impl Crawler {
    pub fn new() -> Self {
        Self {}
    }
}

/// Crawl trait allows to have any Client (sync as of now)
pub trait CrawlSimple {
    fn crawl(&self, url: String, client: &dyn ClientInterface<Vec<String>, ClientError>) -> Result<Vec<String>, ClientError>;
}

impl CrawlSimple for Crawler {
    fn crawl(&self, url: String, client: &dyn ClientInterface<Vec<String>, ClientError>) -> Result<Vec<String>, ClientError> {
        let v = client.get(&url)?;
        Ok(v)
    }
}
