# baidu_trans
百度翻译API

# Install
```toml
[dependencies]
baidu_trans =  { version = "0.7.2", features = [] }
anyhow = "1.0.66"
dotenv = "0.15.0"
```

# examples
## `blocking` feature:
```rust

use baidu_trans::{blocking::Client, config::Config, lang::Lang};

fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    let app_id = dotenv::var("APP_ID")?;
    let app_secret = dotenv::var("APP_SECRET")?;

    let client = Client::new(Config::new(app_id, app_secret));

    client.lang(Lang::Auto, Lang::Zh);
    let resp = client.translate(
        "As we discussed in Chapter 1, Hello Rust!, stack variables are preferred thanks to
their low overhead and speed compared to heap-allocated data, which
automatically introduces overhead thanks to the necessary heap pointer.",
    )?;

    if resp.error_code.is_some() {
        return Err(anyhow::Error::msg(resp.error_msg.unwrap()));
    }

    dbg!(resp);

    Ok(())
}
```
## `aio` feature
in `cargo.toml`:
```toml
[dependencies]
anyhow = "1.0.66"
baidu_trans =  { version = "0.7.2", features = [ "aio"] }
tokio = { version = "1.21.2", features = ["full"] }
dotenv = "0.15.0"
```

```rust
use baidu_trans::{aio::Client, config::Config, lang::Lang};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    let app_id = dotenv::var("APP_ID")?;
    let app_secret = dotenv::var("APP_SECRET")?;

    let client = Client::new(Config::new(app_id, app_secret));

    client.lang(Lang::Auto, Lang::Zh);
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

    Ok(())
}
```

# features
- `blocking`: 默认特性
- `aio`: 启动`async/await`功能支持
- `image`: 启用图片翻译功能
- `domain`: 启用垂直领域翻译功能
- `doc`:启用文档翻译功能