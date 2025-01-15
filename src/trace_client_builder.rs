use crate::{Error, Result, TraceClient};
use reqwest::ClientBuilder;

#[derive(Default)]
pub struct TraceClientBuilder(pub ClientBuilder);

impl TraceClientBuilder {
    pub fn new() -> Self {
        Self(ClientBuilder::new())
    }

    pub fn build(self) -> Result<TraceClient> {
        self.0.build().map(TraceClient).map_err(Error::Reqwest)
    }

    pub fn redirect(self, policy: reqwest::redirect::Policy) -> Self {
        Self(self.0.redirect(policy))
    }
}
