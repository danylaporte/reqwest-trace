use reqwest::StatusCode;
use std::fmt::{self, Debug, Display};

pub enum Error {
    #[cfg(feature = "json")]
    Json(serde_json::Error),
    Reqwest(reqwest::Error),
    StatusError(StatusCode),
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(feature = "json")]
            Self::Json(e) => Debug::fmt(e, f),
            Self::Reqwest(e) => Debug::fmt(e, f),
            Self::StatusError(e) => Debug::fmt(e, f),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(feature = "json")]
            Self::Json(e) => Display::fmt(e, f),
            Self::Reqwest(e) => Display::fmt(e, f),
            Self::StatusError(e) => Display::fmt(e, f),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
