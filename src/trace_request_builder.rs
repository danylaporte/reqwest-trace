use crate::{trace_client::TraceClient, Error, Result, TraceResponse};
use http::{HeaderMap, Version};
use reqwest::{
    header::{HeaderName, HeaderValue},
    Body, RequestBuilder,
};
use serde::Serialize;
use std::{fmt::Display, time::Duration};

pub struct TraceRequestBuilder(pub RequestBuilder);

impl TraceRequestBuilder {
    pub fn basic_auth<U, P>(self, username: U, password: Option<P>) -> Self
    where
        U: Display,
        P: Display,
    {
        Self(self.0.basic_auth(username, password))
    }

    pub fn bearer_auth<T>(self, token: T) -> Self
    where
        T: Display,
    {
        Self(self.0.bearer_auth(token))
    }

    /// Set the request body.
    pub fn body<T: Into<Body>>(self, body: T) -> Self {
        Self(self.0.body(body))
    }

    pub fn form<T: Serialize + ?Sized>(self, form: &T) -> Self {
        Self(self.0.form(form))
    }

    pub fn header<K, V>(self, key: K, value: V) -> Self
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<http::Error>,
    {
        Self(self.0.header(key, value))
    }

    pub fn headers(self, headers: HeaderMap) -> Self {
        Self(self.0.headers(headers))
    }

    #[cfg(feature = "json")]
    #[cfg_attr(docsrs, doc(cfg(feature = "json")))]
    pub fn json<T: Serialize + ?Sized>(self, json: &T) -> Self {
        Self(self.0.json(json))
    }

    #[cfg(feature = "multipart")]
    #[cfg_attr(docsrs, doc(cfg(feature = "multipart")))]
    pub fn multipart(self, multipart: reqwest::multipart::Form) -> Self {
        Self(self.0.multipart(multipart))
    }

    pub fn query<T: Serialize + ?Sized>(self, query: &T) -> Self {
        Self(self.0.query(query))
    }

    pub async fn send(self) -> Result<TraceResponse> {
        let req = self.0.build().map_err(Error::Reqwest)?;
        TraceClient::new().execute(req).await
    }

    pub fn timeout(self, timeout: Duration) -> Self {
        Self(self.0.timeout(timeout))
    }

    pub fn try_clone(&self) -> Option<Self> {
        self.0.try_clone().map(Self)
    }

    pub fn version(self, version: Version) -> Self {
        Self(self.0.version(version))
    }
}

impl From<RequestBuilder> for TraceRequestBuilder {
    fn from(b: RequestBuilder) -> Self {
        Self(b)
    }
}
