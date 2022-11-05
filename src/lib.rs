//! 百度API翻译SDK
//!
//! ## 支持的功能
//! - [x] 文本翻译
//! - [x] 图片翻译
//!
//! 引入依赖:
//!
//! ```ini
//! [dependencies]
//! baidu_trans =  { version = "0.5.2", features = ["image"] }
//! ```
//!
//! 如果要使用`async/await`，需要添加`aio` features。
//!
//! 如果要支持图片翻译，需要添加`image` feature。
//!
//! # 基本用法
//!
//! ```ini
//! [dependencies]
//! anyhow = "1.0.66"
//! baidu_trans =  { version = "0.5.2", features = ["image"] }
//! tokio = { version = "1.21.2", features = ["full"] }
//! dotenv = "0.15.0"
//! ```
//!
//! ## 使用阻塞的方式。
//!
//! ```rust,no_run,ignore
//! use std::fs;
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
//!     if resp.error_code.is_some() {
//!         return Err(anyhow::Error::msg(resp.error_msg.unwrap()));
//!     }
//!
//!     dbg!(resp);
//!
//!     let data = fs::read("a.png")?;
//!     client.lang(Lang::Auto, Lang::Zh);
//!     let resp = client.image_translate("a.png", data)?;
//!
//!     dbg!(resp);
//!     Ok(())
//! }
//! ```
//!
//! ## 使用`async/await`方式
//!
//! ```ini
//! [dependencies]
//! anyhow = "1.0.66"
//! baidu_trans =  { version = "0.5.2", features = ["aio", "image"] }
//! tokio = { version = "1.21.2", features = ["full"] }
//! dotenv = "0.15.0"
//! ```
//!
//! 需要在`cargo.toml`中添加`aio` features。
//!
//! ```rust,no_run,ignore
//! use baidu_trans::{aio::Client, config::Config, lang::Lang};
//! use tokio::fs;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     dotenv::dotenv()?;
//!     let app_id = dotenv::var("APP_ID")?;
//!     let app_secret = dotenv::var("APP_SECRET")?;
//!
//!     let client = Client::new(Config::new(app_id, app_secret));
//!
//!     let resp = client
//!         .translate(
//!             "As we discussed in Chapter 1, Hello Rust!, stack variables are preferred thanks to
//! their low overhead and speed compared to heap-allocated data, which
//! automatically introduces overhead thanks to the necessary heap pointer.",
//!         )
//!         .await?;
//!
//!     if resp.error_code.is_some() {
//!         return Err(anyhow::Error::msg(resp.error_msg.unwrap()));
//!     }
//!
//!     dbg!(resp);
//!
//!     let data = fs::read("a.png").await?;
//!     client.lang(Lang::Auto, Lang::Zh);
//!     let resp = client.image_translate("a.png", data).await?;
//!
//!     dbg!(resp);
//!
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]

#[cfg(feature = "aio")]
pub mod aio;
#[cfg(feature = "blocking")]
pub mod blocking;
pub mod config;
pub mod constant;
pub mod lang;
pub mod model;
pub mod util;
