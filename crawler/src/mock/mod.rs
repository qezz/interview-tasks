mod hashmap;
pub use hashmap::*;

mod slowed_sync;
pub(crate) use slowed_sync::SlowedClient;

mod slowed_async;
pub use slowed_async::*;


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn simple() {
        use crate::simple::{Crawler, CrawlSimple};

        let fake_data: HashMap<String, Vec<String>> = vec![
            ("1".into(), vec!["2".into(), "3".into()]),
        ].into_iter().collect();

        let crawler = Crawler::new();
        let res = crawler.crawl("1".into(), &fake_data).unwrap();
        assert_eq!(vec!["2".to_string(), "3".to_string()], res);
    }
}
