use reqwest::StatusCode;
use std::fmt::{self, Debug, Display};

pub enum Error {
    #[cfg(feature = "json")]
    Json(serde_json::Error),
    Reqwest(reqwest::Error),
    StatusError(StatusCode),
}

impl Error {
    pub fn is_connect(&self) -> bool {
        match self {
            Self::Reqwest(r) => r.is_connect(),
            _ => false,
        }
    }

    pub fn is_request(&self) -> bool {
        match self {
            #[cfg(feature = "json")]
            Json(_) => false,
            Self::Reqwest(r) => r.is_request(),
            Self::StatusError(s) => s.is_client_error(),
        }
    }

    pub fn is_status(&self) -> bool {
        match self {
            #[cfg(feature = "json")]
            Json(_) => false,
            Self::Reqwest(r) => r.is_status(),
            Self::StatusError(_) => true,
        }
    }

    pub fn is_timeout(&self) -> bool {
        match self {
            Self::Reqwest(r) => r.is_timeout(),
            _ => false,
        }
    }

    pub fn status(&self) -> Option<StatusCode> {
        match self {
            #[cfg(feature = "json")]
            Json(_) => None,
            Self::Reqwest(r) => r.status(),
            Self::StatusError(c) => Some(*c),
        }
    }
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
