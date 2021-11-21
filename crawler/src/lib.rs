mod mock;
mod common;

mod simple;
mod better;
mod v3_better;
pub mod v4_tokio_wrong;
pub mod v5_tokio_better;

pub use better::crawl;
pub use v3_better::crawl as v3_crawl;
// pub use v4_better;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
