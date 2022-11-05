# baidu_trans
百度翻译API

# Install
```toml
[dependencies]
baidu_trans =  { version = "0.3.0", features = ["image"] }

[dev-dependencies]
dotenv = "0.15.0"
```

# examples
## `blocking` feature:
```rust
use std::fs;

use baidu_trans::{bloking::Client, config::Config};

fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    let app_id = dotenv::var("APP_ID")?;
    let app_secret = dotenv::var("APP_SECRET")?;

    let client = Client::new(Config::new(app_id, app_secret));

    let resp = client.translate(
        "As we discussed in Chapter 1, Hello Rust!, stack variables are preferred thanks to
their low overhead and speed compared to heap-allocated data, which
automatically introduces overhead thanks to the necessary heap pointer.",
    )?;

    if resp.error_code.is_some() {
        return Err(anyhow::Error::msg(resp.error_msg.unwrap()));
    }

    dbg!(resp);

    let data = fs::read("a.png")?;
    client.lang("auto", "zh");
    let resp = client.image_translate("a.png", data)?;

    dbg!(resp);
    Ok(())
}
```
## `aio` feature
in `cargo.toml`:
```toml
[dependencies]
anyhow = "1.0.66"
baidu_trans =  { version = "0.3.0", default-features = false, features = [ "aio", "image"] }
tokio = { version = "1.21.2", features = ["full"] }

[dev-dependencies]
dotenv = "0.15.0"

```

```rust
use baidu_trans::{aio::Client, config::Config};
use tokio::fs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    let app_id = dotenv::var("APP_ID")?;
    let app_secret = dotenv::var("APP_SECRET")?;

    let client = Client::new(Config::new(app_id, app_secret));

    let resp = client
        .translate(
            "As we discussed in Chapter 1, Hello Rust!, stack variables are preferred thanks to
their low overhead and speed compared to heap-allocated data, which
automatically introduces overhead thanks to the necessary heap pointer.",
        )
        .await?;

    if resp.error_code.is_some() {
        return Err(anyhow::Error::msg(resp.error_msg.unwrap()));
    }

    dbg!(resp);

    let data = fs::read("a.png").await?;
    client.lang("auto", "zh");
    let resp = client.image_translate("a.png", data).await?;

    dbg!(resp);

    Ok(())
}
```