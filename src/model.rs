//! 通用翻译相关模块

use serde::Deserialize;

/// 通用翻译返回结构
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Eq, Deserialize)]
pub struct CommonResult {
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
    /// 原文tts链接，mp3格式，暂时无法指定发音
    pub src_tts: Option<String>,
    /// 译文tts链接，mp3格式，暂时无法指定发音
    pub dst_tts: Option<String>,
    /// 中英词典资源，返回中文或英文词典资源，包含音标；简明释义等内容
    pub dict: Option<String>,
}

/// 翻译返回的结果
#[derive(Debug, Clone, PartialEq, Default, Eq, Deserialize)]
pub struct TranslateResult {
    /// 源文本
    pub src: String,
    /// 翻译后的文本
    pub dst: String,
}
