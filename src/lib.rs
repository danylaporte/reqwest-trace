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
mod trace_request_builder;
mod trace_response;
mod utils;

pub use error::{Error, Result};
pub use reqwest::{self, header, StatusCode};
pub use trace_client::TraceClient;
pub use trace_request_builder::TraceRequestBuilder;
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
