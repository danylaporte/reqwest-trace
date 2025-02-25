use crate::{utils::headers_repr, Error, Result};
use reqwest::{Response, StatusCode};
use tracing::{error, trace, Instrument, Span};

pub struct TraceResponse(pub(crate) Response, pub(crate) Span);

impl TraceResponse {
    pub async fn bytes(self) -> Result<bytes::Bytes> {
        self.bytes_imp(true).await.map(|(v, _headers, _span)| v)
    }

    async fn bytes_imp(self, log_success: bool) -> Result<(bytes::Bytes, HeadersRepr, Span)> {
        let headers = headers_repr(self.0.headers());
        let span = self.1;

        match self.0.bytes().instrument(span.clone()).await {
            Ok(v) => {
                if log_success {
                    span.in_scope(|| trace!(headers = headers, size = v.len(), "bytes"));
                }

                Ok((v, headers, span))
            }
            Err(e) => {
                span.record("error", true);
                span.record("error_description", e.to_string());
                span.in_scope(|| error!(res_headers = headers, "bytes"));

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

            span.in_scope(|| error!(headers = headers_repr(self.0.headers())));
            span.record("error", true);

            match self.0.text().instrument(span.clone()).await {
                Ok(t) => {
                    span.record(
                        "error_description",
                        t.chars().take(1024).collect::<String>(),
                    );
                    Err(Error::StatusError(status))
                }
                Err(e) => {
                    span.record("error_description", e.to_string());
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
        use bytes::Bytes;

        let (full, headers, span) = self.bytes_imp(false).await?;

        fn trace_ok(span: Span, headers: String, bytes: Bytes) {
            span.in_scope(|| {
                trace!(
                    headers = headers,
                    size = bytes.len(),
                    text = %text_repr(&bytes),
                    "json"
                )
            });
        }

        fn trace_err(span: Span, headers: String, bytes: Bytes, error: serde_json::Error) -> Error {
            span.record("error", true);
            span.record("error_description", error.to_string());

            span.in_scope(|| {
                error!(
                    headers = headers,
                    size = bytes.len(),
                    text = %text_repr(&bytes),
                    "json"
                )
            });

            Error::Json(error)
        }

        match serde_json::from_slice::<T>(&full) {
            Ok(v) => {
                trace_ok(span, headers, full);
                Ok(v)
            }
            Err(e) => Err(trace_err(span, headers, full, e)),
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
                span.in_scope(|| {
                    trace!(
                        headers = headers,
                        size = v.len(),
                        text = v.chars().take(256).collect::<String>(),
                        "text"
                    );
                });

                Ok(v)
            }
            Err(e) => {
                span.record("error", true);
                span.record("error_description", e.to_string());
                span.in_scope(|| error!(headers = headers, "text"));

                Err(Error::Reqwest(e))
            }
        }
    }
}

type HeadersRepr = String;
