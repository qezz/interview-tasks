#[derive(Debug)]
pub enum ClientError {
    Request(reqwest::Error),

    /// This Fake error is used for tests
    Fake,
}

impl From<reqwest::Error> for ClientError {
    fn from(e: reqwest::Error) -> ClientError {
        ClientError::Request(e)
    }
}
