#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("reqwest error")]
    Reqwest(#[from] reqwest::Error),
    #[error("feed_rs error")]
    FeedRs(#[from] feed_rs::parser::ParseFeedError),
    #[error("std::io error")]
    Io(#[from] std::io::Error),
    #[error("serde_json error")]
    Json(#[from] serde_json::Error),
}
