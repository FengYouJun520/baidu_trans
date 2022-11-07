//! 相关辅助函数
use std::collections::HashMap;

use chrono::Local;
use md5::{Digest, Md5};

use crate::config::Config;

/// 构建表单参数
macro_rules! build_params {
    ($(($key: literal, $value: expr)),+ $(,)?) => {{
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

/// 构建文档翻译统计校验服务表单
#[cfg(all(feature = "blocking", feature = "doc"))]
pub(crate) fn build_doc_count_form_blocking(
    config: &Config,
    data: Vec<u8>,
    name: &str,
    ext: &str,
) -> anyhow::Result<reqwest::blocking::multipart::Form> {
    use reqwest::blocking::multipart;

    let mut kv = vec![
        ("appid".to_string(), config.app_id.to_string()),
        ("from".to_string(), config.from.to_string()),
        ("to".to_string(), config.to.to_string()),
        (
            "timestamp".to_string(),
            Local::now().timestamp().to_string(),
        ),
        ("type".to_string(), ext.to_string()),
    ];
    // 签名需要按key进行排序
    kv.sort_by(|a, b| a.0.cmp(&b.0));

    // 拼接查询参数,最后面必须要有&
    let mut query = String::new();
    for (k, v) in kv.iter() {
        query.push_str(k);
        query.push_str("=");
        query.push_str(v);
        query.push_str("&");
    }

    let sign = md5_encode!(&query, md5_encode!(&data), &config.secret_key);
    // 创建表单
    let mut params = multipart::Form::new();
    for (k, v) in kv {
        params = params.text(k, v);
    }
    params = params.text("sign", sign).part(
        "file",
        multipart::Part::bytes(data).file_name(name.to_string()),
    );

    Ok(params)
}

/// 构建文档翻译统计校验服务表单
#[cfg(all(feature = "aio", feature = "doc"))]
pub(crate) fn build_doc_count_form_aio(
    config: &Config,
    data: Vec<u8>,
    name: &str,
    ext: &str,
) -> anyhow::Result<reqwest::multipart::Form> {
    use reqwest::multipart;

    let mut kv = vec![
        ("appid".to_string(), config.app_id.to_string()),
        ("from".to_string(), config.from.to_string()),
        ("to".to_string(), config.to.to_string()),
        (
            "timestamp".to_string(),
            Local::now().timestamp().to_string(),
        ),
        ("type".to_string(), ext.to_string()),
    ];
    // 签名需要按key进行排序
    kv.sort_by(|a, b| a.0.cmp(&b.0));

    // 拼接查询参数,最后面必须要有&
    let mut query = String::new();
    for (k, v) in kv.iter() {
        query.push_str(k);
        query.push_str("=");
        query.push_str(v);
        query.push_str("&");
    }

    let sign = md5_encode!(&query, md5_encode!(&data), &config.secret_key);
    // 创建表单
    let mut params = multipart::Form::new();
    for (k, v) in kv {
        params = params.text(k, v);
    }
    params = params.text("sign", sign).part(
        "file",
        multipart::Part::bytes(data).file_name(name.to_string()),
    );

    Ok(params)
}

/// 构建文档翻译服务表单
#[cfg(all(feature = "blocking", feature = "doc"))]
pub(crate) fn build_doc_form_blocking(
    config: &Config,
    data: Vec<u8>,
    name: &str,
    typ: &str,
    out_type: &str,
) -> anyhow::Result<reqwest::blocking::multipart::Form> {
    use reqwest::blocking::multipart;

    let mut kv = vec![
        ("appid".to_string(), config.app_id.to_string()),
        ("from".to_string(), config.from.to_string()),
        ("to".to_string(), config.to.to_string()),
        (
            "timestamp".to_string(),
            Local::now().timestamp().to_string(),
        ),
        ("type".to_string(), typ.to_string()),
        ("outPutType".to_string(), out_type.to_string()),
    ];
    // 签名需要按key进行排序
    kv.sort_by(|a, b| a.0.cmp(&b.0));

    // 拼接查询参数,最后面必须要有&
    let mut query = String::new();
    for (k, v) in kv.iter() {
        query.push_str(k);
        query.push_str("=");
        query.push_str(v);
        query.push_str("&");
    }

    let sign = md5_encode!(&query, md5_encode!(&data), &config.secret_key);

    // 创建表单
    let mut params = multipart::Form::new();
    for (k, v) in kv {
        params = params.text(k, v);
    }

    params = params.text("sign".to_string(), sign).part(
        "file",
        multipart::Part::bytes(data).file_name(name.to_string()),
    );

    Ok(params)
}

/// 构建文档翻译服务表单
#[cfg(all(feature = "aio", feature = "doc"))]
pub(crate) fn build_doc_form_aio(
    config: &Config,
    data: Vec<u8>,
    name: &str,
    typ: &str,
    out_type: &str,
) -> anyhow::Result<reqwest::multipart::Form> {
    use reqwest::multipart;

    let mut kv = vec![
        ("appid".to_string(), config.app_id.to_string()),
        ("from".to_string(), config.from.to_string()),
        ("to".to_string(), config.to.to_string()),
        (
            "timestamp".to_string(),
            Local::now().timestamp().to_string(),
        ),
        ("type".to_string(), typ.to_string()),
        ("outPutType".to_string(), out_type.to_string()),
    ];
    // 签名需要按key进行排序
    kv.sort_by(|a, b| a.0.cmp(&b.0));

    // 拼接查询参数,最后面必须要有&
    let mut query = String::new();
    for (k, v) in kv.iter() {
        query.push_str(k);
        query.push_str("=");
        query.push_str(v);
        query.push_str("&");
    }

    let sign = md5_encode!(&query, md5_encode!(&data), &config.secret_key);

    // 创建表单
    let mut params = multipart::Form::new();
    for (k, v) in kv {
        params = params.text(k, v);
    }

    params = params.text("sign".to_string(), sign).part(
        "file",
        multipart::Part::bytes(data).file_name(name.to_string()),
    );

    Ok(params)
}
