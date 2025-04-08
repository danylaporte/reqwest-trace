//! Tracing wrapper for Reqwest.
//!
//! ```
//!  use reqwest_trace::{Result, TraceClient};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let client = TraceClient::new();
//!
//!     let text = client
//!         .get("http://www.google.com")
//!         .send()
//!         .await?
//!         .text()
//!         .await?;
//!
//!     Ok(())
//! }
//! ```

mod error;
mod trace_client;
mod trace_client_builder;
mod trace_request_builder;
mod trace_response;
mod utils;

pub use error::{Error, Result};
#[cfg(feature = "multipart")]
pub use reqwest::multipart;
pub use reqwest::{self, header, Body, IntoUrl, Method, StatusCode, Version};
pub use trace_client::TraceClient;
pub use trace_client_builder::TraceClientBuilder;
pub use trace_request_builder::{RetryArgs, TraceRequestBuilder};
pub use trace_response::TraceResponse;

#[cfg(test)]
#[tracing_test::traced_test]
#[tokio::test]
async fn check_tracing() -> Result<()> {
    let client = TraceClient::new();
    let _ = client
        .get("http://www.google.com")
        .send()
        .await?
        .text()
        .await?;

    Ok(())
}
