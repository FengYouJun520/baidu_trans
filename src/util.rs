//! 工具函数
use std::collections::HashMap;

use chrono::Local;
use md5::{Digest, Md5};

use crate::config::Config;

/// 构建表单参数
macro_rules! build_params {
    ($(($key: literal, $value: expr)),*) => {{
        let mut params = HashMap::new();
        $(
            params.insert($key.to_string(), $value.to_string());
        )*
        params
    }};
}

/// md5加密
macro_rules! md5_encode {
    ($($item: expr),+ $(,)?) => {{
        let mut hasher = Md5::new();

        $(
            hasher = hasher.chain_update($item);
        )*

        format!("{:x}", hasher.finalize())
    }};
}

/// 构建通用翻译表单
/// - config: 客户端配置
/// - q: 待翻译的文本
pub(crate) fn build_form(config: &Config, q: &str) -> HashMap<String, String> {
    let salt = Local::now().timestamp();
    let mut params = build_params! {
        ("q", q),
        ("from", &config.from),
        ("to", &config.to),
        ("appid", config.app_id),
        ("salt", salt)
    };

    // let sign = sign_q(config, q, salt);
    let sign = md5_encode!(&config.app_id, q, salt.to_string(), &config.secret_key);
    params.insert("sign".into(), sign);

    // 开通词典、TTS用户
    if config.open_dict || config.open_tts {
        params.insert("tts".into(), 1.to_string());
        params.insert("dict".into(), 1.to_string());
    }

    // 是否开通了"我的术语"
    if config.open_action {
        params.insert("action".into(), 1.to_string());
    }

    params
}

/// 构建图片翻译表单参数, 需要开启`aio`和`image` features。
/// - name: 文件名
/// - data: 图片数据
/// - config: 客户端配置
/// - cuid: 固定值：APICUID
/// - mac: 固定值：mac
/// - version: 固定值：3
/// - paste: 图片贴合类型：0 - 关闭文字贴合 、1 - 返回整图贴合 、2 - 返回块区贴合
#[cfg(all(feature = "aio", feature = "image"))]
pub(crate) fn create_image_form(
    name: &str,
    data: Vec<u8>,
    config: &Config,
    cuid: &str,
    mac: &str,
    version: &str,
    paste: Option<String>,
) -> reqwest::multipart::Form {
    use reqwest::multipart;

    let salt = Local::now().timestamp();
    let sign = md5_encode!(
        &config.app_id,
        md5_encode!(&data),
        salt.to_string(),
        cuid,
        mac,
        &config.secret_key,
    );

    let params = multipart::Form::new()
        .part(
            "image",
            multipart::Part::bytes(data).file_name(name.to_string()),
        )
        .text("from", config.from.to_string())
        .text("to", config.to.to_string())
        .text("appid", config.app_id.clone())
        .text("salt", salt.to_string())
        .text("cuid", cuid.to_string())
        .text("mac", mac.to_string())
        .text("version", version.to_string())
        .text("sign", sign);

    if let Some(paste) = paste {
        params.text("paste", paste)
    } else {
        params
    }
}

/// 构建图片翻译表单参数，需要开启`blocking`和`image` features。
/// - name: 文件名
/// - data: 图片数据
/// - config: 客户端配置
/// - cuid: 固定值：APICUID
/// - mac: 固定值：mac
/// - version: 固定值：3
/// - paste: 图片贴合类型：0 - 关闭文字贴合 、1 - 返回整图贴合 、2 - 返回块区贴合
#[cfg(all(feature = "blocking", feature = "image"))]
pub(crate) fn create_image_form_blocking(
    name: &str,
    data: Vec<u8>,
    config: &Config,
    cuid: &str,
    mac: &str,
    version: &str,
    paste: Option<String>,
) -> reqwest::blocking::multipart::Form {
    use reqwest::blocking::multipart;

    let salt = Local::now().timestamp();
    let sign = md5_encode!(
        &config.app_id,
        md5_encode!(&data),
        salt.to_string(),
        cuid,
        mac,
        &config.secret_key
    );

    let params = multipart::Form::new()
        .part(
            "image",
            multipart::Part::bytes(data).file_name(name.to_string()),
        )
        .text("from", config.from.to_string())
        .text("to", config.to.to_string())
        .text("appid", config.app_id.clone())
        .text("salt", salt.to_string())
        .text("cuid", cuid.to_string())
        .text("mac", mac.to_string())
        .text("version", version.to_string())
        .text("sign", sign);

    if let Some(paste) = paste {
        params.text("paste", paste)
    } else {
        params
    }
}

/// 构建垂直领域翻译表单
/// - config: 客户端配置
/// - q: 待翻译的文本
/// - domain: 所选择的垂直领域
#[cfg(feature = "domain")]
pub(crate) fn build_domain_form(
    config: &Config,
    q: &str,
    domain: crate::domain::Domain,
) -> HashMap<String, String> {
    let mut params = HashMap::new();

    let salt = Local::now().timestamp();
    params.insert("q".into(), q.into());
    params.insert("from".into(), config.from.to_string());
    params.insert("to".into(), config.to.to_string());
    params.insert("appid".into(), config.app_id.clone());
    params.insert("salt".into(), salt.to_string());
    params.insert("domain".into(), domain.to_string());

    let sign = md5_encode!(
        &config.app_id,
        q,
        salt.to_string(),
        domain.to_string(),
        &config.secret_key
    );
    params.insert("sign".into(), sign);

    params
}

#[cfg(feature = "doc")]
pub(crate) fn build_doc_form<T: AsRef<[u8]>>(config: &Config, data: T) -> HashMap<String, String> {
    let mut params = get_params();
    params
}

#[cfg(feature = "doc")]
pub(crate) fn get_params() -> HashMap<String, String> {
    let params = HashMap::new();
    let mut hasher = Md5::new();
    params
}

/// 对文件进行md5加密
pub(crate) fn mod_file<T: AsRef<[u8]>>(data: T) -> String {
    let mut hasher = Md5::new();
    hasher.update(data);

    format!("{:x}", hasher.finalize())
}
