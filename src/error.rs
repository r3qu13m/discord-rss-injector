#[derive(Debug)]
pub enum Error {
    Reqwest(String),
    FeedRs(String),
    Io(String),
    Json(String),
}

macro_rules! impl_error {
    ($name: ident, $target: ty) => {
        impl From<$target> for Error {
            fn from(value: $target) -> Self {
                Self::$name(value.to_string())
            }
        }
    };
}

impl_error!(Reqwest, reqwest::Error);
impl_error!(FeedRs, feed_rs::parser::ParseFeedError);
impl_error!(Io, std::io::Error);
impl_error!(Json, serde_json::Error);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Reqwest(e) => write!(f, "Reqwest error: {}", e),
            Self::FeedRs(e) => write!(f, "feed_rs error: {}", e),
            Self::Io(e) => write!(f, "std::io error: {}", e),
            Self::Json(e) => write!(f, "serde_json error: {}", e),
        }
    }
}

impl std::error::Error for Error {}
