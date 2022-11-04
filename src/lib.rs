pub mod model;
use std::{cell::RefCell, collections::HashMap};

use chrono::Local;
use md5::{Digest, Md5};
use model::*;

/// 百度通用翻译API地址
const COMMON_URL: &'static str = "https://fanyi-api.baidu.com/api/trans/vip/translate";

/// 百度翻译客户端
pub struct Client {
    config: RefCell<Config>,
    #[cfg(feature = "blocking")]
    http_client: reqwest::blocking::Client,
    #[cfg(feature = "aio")]
    http_client: reqwest::Client,
}

impl Client {
    pub fn new(config: Config) -> Self {
        Self {
            config: RefCell::new(config),
            #[cfg(feature = "blocking")]
            http_client: reqwest::blocking::Client::builder().build().unwrap(),
            #[cfg(feature = "aio")]
            http_client: reqwest::Client::builder().build().unwrap(),
        }
    }

    /// 设置源语言和目标语言
    pub fn language(&self, from: &str, to: &str) {
        self.config.borrow_mut().set_from(from);
        self.config.borrow_mut().set_to(to);
    }

    /// 对app_key, app_secret, q, salt进行签名
    fn sign_q<T: AsRef<str>>(&self, q: T, salt: i64) -> String {
        let mut sign_str = String::new();
        sign_str.push_str(&self.config.borrow().app_id);
        sign_str.push_str(q.as_ref());
        sign_str.push_str(&salt.to_string());
        sign_str.push_str(&self.config.borrow().app_secret);

        let mut hasher = Md5::new();
        hasher.update(sign_str);
        return format!("{:x}", hasher.finalize());
    }

    fn build_form(&self, q: &str) -> HashMap<String, String> {
        let mut params = HashMap::new();

        let salt = Local::now().timestamp();
        params.insert("q".into(), q.into());
        params.insert("from".into(), self.config.borrow().from.clone());
        params.insert("to".into(), self.config.borrow().to.clone());
        params.insert("appid".into(), self.config.borrow().app_id.clone());
        params.insert("salt".into(), salt.to_string());

        let sign = self.sign_q(q, salt);
        params.insert("sign".into(), sign);

        // 开通词典、TTS用户
        if self.config.borrow().open_dict || self.config.borrow().open_tts {
            params.insert("tts".into(), 1.to_string());
            params.insert("dict".into(), 1.to_string());
        }

        // 是否开通了"我的术语"
        if self.config.borrow().open_action {
            params.insert("action".into(), 1.to_string());
        }

        params
    }
}

#[cfg(feature = "blocking")]
impl Client {
    pub fn translate(&self, q: &str) -> anyhow::Result<CommonResult> {
        let params = self.build_form(q);

        let resp = self.http_client.post(COMMON_URL).form(&params).send()?;

        Ok(resp.json()?)
    }
}

#[cfg(feature = "aio")]
impl Client {
    pub async fn translate(&self, q: &str) -> anyhow::Result<CommonResult> {
        let params = self.build_form(q);

        let resp = self
            .http_client
            .post(COMMON_URL)
            .form(&params)
            .send()
            .await?;

        Ok(resp.json().await?)
    }
}

/// 客户端配置
pub struct Config {
    /// APP ID
    app_id: String,
    /// 密钥
    app_secret: String,
    /// 源语言，默认auto
    from: String,
    /// 目标语言，默认auto
    to: String,
    /// 是否开通词典
    open_dict: bool,
    /// 是否开通了TTS
    open_tts: bool,
    // 是否开通了"我的术语"
    open_action: bool,
}

impl Config {
    pub fn new(app_id: String, app_secret: String) -> Self {
        Self {
            app_id,
            app_secret,
            from: "auto".into(),
            to: "auto".into(),
            open_dict: false,
            open_tts: false,
            open_action: false,
        }
    }

    pub fn set_from(&mut self, from: &str) {
        self.from = from.to_owned();
    }

    pub fn set_to(&mut self, to: &str) {
        self.to = to.to_owned();
    }
}

#[cfg(feature = "blocking")]
#[test]
fn common_translate() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    let app_id = dotenv::var("APP_ID")?;
    let app_secret = dotenv::var("APP_SECRET")?;

    let client = Client::new(Config::new(app_id, app_secret));

    let resp = client.translate(
        "As we discussed in Chapter 1, Hello Rust!, stack variables are preferred thanks to
their low overhead and speed compared to heap-allocated data, which
automatically introduces overhead thanks to the necessary heap pointer. For
stack variables, Rust's types even allow for zero overhead structures, so no
additional metadata is stored. The following snippet asserts that there are no
additional bytes being used for arrays or user-defined types.",
    )?;

    if resp.error_code.is_some() {
        return Err(anyhow::Error::msg(resp.error_msg.unwrap()));
    }

    let res: Vec<String> = resp
        .trans_result
        .unwrap()
        .iter()
        .map(|t| t.dst.clone())
        .collect();
    let res = res.join("");
    println!("{}", res);
    Ok(())
}

#[cfg(feature = "aio")]
#[tokio::test]
async fn common_translate() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    let app_id = dotenv::var("APP_ID")?;
    let app_secret = dotenv::var("APP_SECRET")?;

    let client = Client::new(Config::new(app_id, app_secret));

    let resp = client
        .translate(
            "As we discussed in Chapter 1, Hello Rust!, stack variables are preferred thanks to
their low overhead and speed compared to heap-allocated data, which
automatically introduces overhead thanks to the necessary heap pointer. For
stack variables, Rust's types even allow for zero overhead structures, so no
additional metadata is stored. The following snippet asserts that there are no
additional bytes being used for arrays or user-defined types.",
        )
        .await?;

    if resp.error_code.is_some() {
        return Err(anyhow::Error::msg(resp.error_msg.unwrap()));
    }

    let res: Vec<String> = resp
        .trans_result
        .unwrap()
        .iter()
        .map(|t| t.dst.clone())
        .collect();
    let res = res.join("");
    println!("{}", res);
    Ok(())
}
