//! 领域翻译返回相关结构

use serde::Deserialize;

use crate::model::TranslateResult;

/// 垂直领域返回结构
#[derive(Debug, Clone, PartialEq, Default, Eq, Deserialize)]
pub struct DomainResult {
    /// 源语言
    pub from: Option<String>,
    /// 目标语言
    pub to: Option<String>,
    /// 翻译结果
    pub trans_result: Option<Vec<TranslateResult>>,
    /// 错误码
    pub error_code: Option<String>,
    /// 错误消息
    pub error_msg: Option<String>,
}
