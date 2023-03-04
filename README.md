# reqwest-trace

A wrapper for reqwest to trace requests (with body).


## Example

```rust

use reqwest_trace::{Result, TraceClient};

#[tokio::main]
async fn main() -> Result<()> {
    let client = TraceClient::new();

    let text = client
        .get("http://www.google.com")
        .send()
        .await?
        .text()
        .await?;

    Ok(())
}

```

## License

The MIT license.
