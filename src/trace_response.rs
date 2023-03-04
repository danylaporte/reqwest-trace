use crate::{utils::headers_repr, Error, Result};
use reqwest::{Response, StatusCode};
use tracing::{error, trace, Instrument, Span};

pub struct TraceResponse(pub(crate) Response, pub(crate) Span);

impl TraceResponse {
    pub async fn bytes(self) -> Result<bytes::Bytes> {
        self.bytes_imp(true).await.map(|(v, _headers)| v)
    }

    async fn bytes_imp(self, log_success: bool) -> Result<(bytes::Bytes, HeadersRepr)> {
        let headers = headers_repr(self.0.headers());
        let span = self.1;

        match self.0.bytes().instrument(span.clone()).await {
            Ok(v) => {
                if log_success {
                    trace!(parent: span, headers = headers, size = v.len(),);
                }

                Ok((v, headers))
            }
            Err(e) => {
                span.record("error", true);
                span.record("error_message", e.to_string());

                error!(parent: span, res_headers = headers);

                Err(Error::Reqwest(e))
            }
        }
    }

    pub async fn error_for_status(self) -> Result<Self> {
        let status = self.0.status();

        if status.is_success() {
            Ok(self)
        } else {
            let span = self.1;

            span.record("error", true);
            error!(parent: span.clone(), headers = headers_repr(self.0.headers()));

            match self.0.text().instrument(span.clone()).await {
                Ok(t) => {
                    span.record("error_message", t.chars().take(1024).collect::<String>());

                    Err(Error::StatusError(status))
                }
                Err(e) => {
                    span.record("error_message", e.to_string());
                    Err(Error::Reqwest(e))
                }
            }
        }
    }

    pub fn headers(&self) -> &reqwest::header::HeaderMap {
        self.0.headers()
    }

    #[cfg(feature = "json")]
    pub async fn json<T: serde::de::DeserializeOwned>(self) -> Result<T> {
        use crate::utils::text_repr;

        let span = self.1.clone();
        let (full, headers) = self.bytes_imp(false).await?;

        match serde_json::from_slice::<T>(&full) {
            Ok(v) => {
                trace!(
                    parent: span,
                    headers = headers,
                    size = full.len(),
                    text = %text_repr(&full)
                );
                Ok(v)
            }
            Err(e) => {
                span.record("error", true);
                span.record("error_description", e.to_string());

                error!(
                    parent: span,
                    headers = headers,
                    size = full.len(),
                    text = %text_repr(&full)
                );

                Err(Error::Json(e))
            }
        }
    }

    pub fn status(&self) -> StatusCode {
        self.0.status()
    }

    pub async fn text(self) -> Result<String> {
        self.text_with_charset("utf-8").await
    }

    pub async fn text_with_charset(self, default_encoding: &str) -> Result<String> {
        let span = self.1;
        let headers = headers_repr(self.0.headers());

        match self
            .0
            .text_with_charset(default_encoding)
            .instrument(span.clone())
            .await
        {
            Ok(v) => {
                trace!(
                    parent: span,
                    headers = headers,
                    size = v.len(),
                    text = v.chars().take(256).collect::<String>(),
                );

                Ok(v)
            }
            Err(e) => {
                span.record("error", true);
                span.record("error_description", e.to_string());

                error!(parent: span, headers = headers);

                Err(Error::Reqwest(e))
            }
        }
    }
}

type HeadersRepr = String;
