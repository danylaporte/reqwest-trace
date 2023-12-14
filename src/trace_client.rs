use crate::{
    trace_request_builder::TraceRequestBuilder,
    utils::{headers_repr, text_repr},
    Result, TraceResponse,
};
use reqwest::{Client, IntoUrl, Method, Request};
use std::borrow::Cow;
use tracing::{field::Empty, info_span, trace, Instrument};

pub struct TraceClient(pub Client);

impl TraceClient {
    pub fn new() -> Self {
        Self(Client::new())
    }

    pub fn delete<U: IntoUrl>(&self, url: U) -> TraceRequestBuilder {
        self.request(Method::DELETE, url)
    }

    pub async fn execute(&self, req: Request) -> Result<TraceResponse> {
        warn_lock_held();
       
        let span = info_span!(
            "reqwest",
            error = Empty,
            error_description = Empty,
            method = req.method().as_str(),
            status = Empty,
            url = req.url().as_str(),
        );

        span.in_scope(|| {
            trace!(
                body = %match req.body() {
                    Some(body) => match body.as_bytes() {
                        Some(bytes) => text_repr(bytes),
                        None => Cow::Borrowed("<streaming>"),
                    },
                    None => Cow::Borrowed("<none>"),
                },
                headers = headers_repr(req.headers()),
                version = ?req.version(),
            );
        });

        match self.0.execute(req).instrument(span.clone()).await {
            Ok(res) => {
                span.record("status", res.status().as_str());
                Ok(TraceResponse(res, span))
            }
            Err(e) => {
                span.record("error", true);
                span.record("error_description", e.to_string());
                Err(crate::Error::Reqwest(e))
            }
        }
    }

    pub fn get<U: IntoUrl>(&self, url: U) -> TraceRequestBuilder {
        self.request(Method::GET, url)
    }

    pub fn head<U: IntoUrl>(&self, url: U) -> TraceRequestBuilder {
        self.request(Method::HEAD, url)
    }

    pub fn patch<U: IntoUrl>(&self, url: U) -> TraceRequestBuilder {
        self.request(Method::PATCH, url)
    }

    pub fn post<U: IntoUrl>(&self, url: U) -> TraceRequestBuilder {
        self.request(Method::POST, url)
    }

    pub fn put<U: IntoUrl>(&self, url: U) -> TraceRequestBuilder {
        self.request(Method::PUT, url)
    }

    pub fn request<U: IntoUrl>(&self, method: Method, url: U) -> TraceRequestBuilder {
        TraceRequestBuilder(self.0.request(method, url))
    }
}

impl Default for TraceClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "async-cell-lock-detect")]
fn warn_lock_held() {
    async_cell_lock::warn_lock_held();
}

#[cfg(not(feature = "async-cell-lock-detect"))]
fn warn_lock_held() {
}