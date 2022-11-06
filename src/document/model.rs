//! 文档翻译请求的返回结构相关模块

use serde::Deserialize;
/// 统计服务返回结果
#[derive(Debug, Clone, PartialEq, Default, Eq, Deserialize)]
pub struct DocCountResult {
    /// 错误码, 这里用serde_json::Value的原因是百度的接口调用成功后返回的是一个整形，
    ///
    /// 但是调用失败返回的确是一个字符串，无语死！
    pub error_code: serde_json::Value,
    /// 错误消息
    pub error_msg: String,
    /// 结果数据
    pub data: Option<DocCountData>,
}

/// 统计服务返回数据
#[derive(Debug, Clone, PartialEq, Default, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocCountData {
    /// 总字符数
    pub char_count: usize,
    /// 文件id
    pub file_id: String,
    /// 消费金额，单位：分
    pub amount: usize,
}

/// 文档翻译返回结果
#[derive(Debug, Clone, PartialEq, Default, Eq, Deserialize)]
pub struct DocResult {
    /// 错误码, 这里用serde_json::Value的原因是百度的接口调用成功后返回的是一个整形，
    ///
    /// 但是调用失败返回的确是一个字符串，无语死！
    pub error_code: serde_json::Value,
    /// 错误消息
    pub error_msg: String,
    /// 结果数据
    pub data: Option<DocData>,
}

/// 文档翻译结果数据
#[derive(Debug, Clone, PartialEq, Default, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocData {
    /// 本次请求id
    pub request_id: String,
}
