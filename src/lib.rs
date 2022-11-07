//! 百度翻译SDK
//!
//! ## 支持的功能
//! - [x] 文本翻译
//! - [x] 图片翻译
//! - [x] 垂直领域翻译
//! - [x] 文档翻译
//!
//! 引入依赖:
//!
//! ```toml
//! [dependencies]
//! baidu_trans =  { version = "0.7.3", features = [] }
//! ```
//!
//! 如果要使用`async/await`，需要添加`aio` features。
//!
//! 如果要支持图片翻译，需要添加`image` feature。
//!
//! # 基本用法
//!
//! ## 使用阻塞的方式。
//!
//! 添加依赖:
//!
//! ```toml
//! [dependencies]
//! anyhow = "1.0.66"
//! baidu_trans =  { version = "0.7.3", features = [] }
//! dotenv = "0.15.0"
//! ```
//!
//! 默认启用的是`blocking` feature.
//!
//! ```rust,no_run
//! use baidu_trans::{blocking::Client, config::Config, lang::Lang};
//!
//! fn main() -> anyhow::Result<()> {
//!     dotenv::dotenv()?;
//!     let app_id = dotenv::var("APP_ID")?;
//!     let app_secret = dotenv::var("APP_SECRET")?;
//!
//!     let client = Client::new(Config::new(app_id, app_secret));
//!
//!     let resp = client.translate(
//!         "As we discussed in Chapter 1, Hello Rust!, stack variables are preferred thanks to
//! their low overhead and speed compared to heap-allocated data, which
//! automatically introduces overhead thanks to the necessary heap pointer.",
//!     )?;
//!
//!     assert_eq!(resp.error_code, None);
//!
//!     Ok(())
//! }
//! ```
//!
#![cfg_attr(
    feature = "aio",
    doc = r##"
 ## 使用`async/await`方式

 添加依赖:

 ```toml
 [dependencies]
 anyhow = "1.0.66"
 baidu_trans =  { version = "0.7.3", features = ["aio"] }
 tokio = { version = "1.21.2", features = ["full"] }
 dotenv = "0.15.0"
 ```

 需要在`cargo.toml`中添加`aio` features。

 ```rust,no_run
 use baidu_trans::{aio::Client, config::Config, lang::Lang};

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

     assert_eq!(resp.error_code, None);

     dbg!(resp);

     Ok(())
 }
 ```
"##
)]
#![cfg_attr(
    feature = "image",
    doc = r##"
## 图片翻译
需要启用`image` feature.
```toml
[dependencies]
baidu_trans =  { version = "0.7.3", features = ["image"] }
```
使用方式
```rust,no_run
use baidu_trans::blocking::Client;
use baidu_trans::config::Config;
use baidu_trans::lang::Lang;
use std::fs;

fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    let app_id = dotenv::var("APP_ID")?;
    let app_secret = dotenv::var("APP_SECRET")?;
    let client = Client::new(Config::new(app_id, app_secret));
    client.lang(Lang::Auto, Lang::Zh);
    let data = fs::read("a.png")?;
    /// 图片名称必须填写
    let resp = client.image_translate("a.png", data)?;
    assert_eq!(resp.error_code, "0");
    Ok(())
}
```
"##
)]
//!
#![cfg_attr(
    feature = "domain",
    doc = r##"
 ## 垂直领域翻译

 需要启用`domain` feature.

 ```toml
 [dependencies]
 baidu_trans =  { version = "0.7.3", features = ["domain"] }

 ```

 用法

 ```rust,no_run
 use baidu_trans::blocking::Client;
 use baidu_trans::config::Config;
 use baidu_trans::domain::Domain;
 use baidu_trans::lang::Lang;

 fn main() -> anyhow::Result<()> {

    dotenv::dotenv()?;
    let app_id = dotenv::var("APP_ID")?;
    let app_secret = dotenv::var("APP_SECRET")?;

    let client = Client::new(Config::new(app_id, app_secret));

    client.lang(Lang::Auto, Lang::Zh);

    let resp = client.domain_translate(
        "As we discussed in Chapter 1, Hello Rust!, stack variables are preferred thanks to
their low overhead and speed compared to heap-allocated data, which
automatically introduces overhead thanks to the necessary heap pointer.",
        Domain::Electronics,
    )?;

    dbg!(resp);
    Ok(())
}
 ```
"##
)]
#![warn(missing_docs)]
#![cfg_attr(docsrs, warn(rustdoc::broken_intra_doc_links))]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "aio")]
pub mod aio;
#[cfg(feature = "blocking")]
pub mod blocking;
pub mod config;
pub mod constant;
#[cfg(feature = "doc")]
pub mod document;
#[cfg(feature = "domain")]
pub mod domain;
#[cfg(feature = "image")]
pub mod image;
pub mod lang;
pub mod model;
pub mod util;
