//! 工具函数
use std::collections::HashMap;

use chrono::Local;
use md5::{Digest, Md5};

use crate::config::Config;

/// 构建通用翻译表单
/// - config: 客户端配置
/// - q: 待翻译的文本
pub(crate) fn build_form(config: &Config, q: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();

    let salt = Local::now().timestamp();
    params.insert("q".into(), q.into());
    params.insert("from".into(), config.from.to_string());
    params.insert("to".into(), config.to.to_string());
    params.insert("appid".into(), config.app_id.clone());
    params.insert("salt".into(), salt.to_string());

    let sign = sign_q(config, q, salt);
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

/// 通用翻译签名
/// - config: 客户端配置
/// - q: 待翻译的文本
/// - salt: 随机盐
pub(crate) fn sign_q<T: AsRef<str>>(config: &Config, q: T, salt: i64) -> String {
    let mut sign_str = String::new();
    sign_str.push_str(&config.app_id);
    sign_str.push_str(q.as_ref());
    sign_str.push_str(&salt.to_string());
    sign_str.push_str(&config.secret_key);

    let mut hasher = Md5::new();
    hasher.update(sign_str);

    format!("{:x}", hasher.finalize())
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
    let sign = sign_image(&data, config, salt, cuid, mac);

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
    let sign = sign_image(&data, config, salt, cuid, mac);

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

/// 图片翻译签名，需要开启`image` feature。
/// - data: 图片数据
/// - config: 客户端配置
/// - salt: 随机盐
/// - cuid: 固定值：APICUID
/// - mac: 固定值：mac
#[cfg(feature = "image")]
pub(crate) fn sign_image<T: AsRef<[u8]>>(
    data: T,
    config: &Config,
    salt: i64,
    cuid: &str,
    mac: &str,
) -> String {
    let mut hasher = Md5::new();
    hasher.update(data);

    let image_md5 = format!("{:x}", hasher.finalize());

    let mut sign_str = String::new();
    sign_str.push_str(&config.app_id);
    sign_str.push_str(&image_md5);
    sign_str.push_str(&salt.to_string());
    sign_str.push_str(cuid);
    sign_str.push_str(mac);
    sign_str.push_str(&config.secret_key);

    let mut hasher = Md5::new();
    hasher.update(sign_str);

    format!("{:x}", hasher.finalize())
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

    let sign = sign_domain(config, q, salt, domain);
    params.insert("sign".into(), sign);

    params
}

/// 通用翻译签名
/// - config: 客户端配置
/// - q: 待翻译的文本
/// - salt: 随机盐
/// - domain: 垂直领域
#[cfg(feature = "domain")]
pub(crate) fn sign_domain<T: AsRef<str>>(
    config: &Config,
    q: T,
    salt: i64,
    domain: crate::domain::Domain,
) -> String {
    let mut sign_str = String::new();
    sign_str.push_str(&config.app_id);
    sign_str.push_str(q.as_ref());
    sign_str.push_str(&salt.to_string());
    sign_str.push_str(&domain.to_string());
    sign_str.push_str(&config.secret_key);

    let mut hasher = Md5::new();
    hasher.update(sign_str);

    format!("{:x}", hasher.finalize())
}
