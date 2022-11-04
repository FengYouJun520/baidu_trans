use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Eq, Deserialize)]
pub struct CommonResult {
    pub from: Option<String>,
    pub to: Option<String>,
    pub trans_result: Option<Vec<TranslateResult>>,
    pub error_code: Option<String>,
    pub error_msg: Option<String>,
    pub src_tts: Option<String>,
    pub dst_tts: Option<String>,
    pub dict: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Eq, Deserialize)]
pub struct TranslateResult {
    pub src: String,
    pub dst: String,
}
