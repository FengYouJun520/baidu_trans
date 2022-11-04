use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Default, Eq, Deserialize)]
pub struct CommonResult {
    pub from: String,
    pub to: String,
    pub trans_result: Vec<TranslateResult>,
    pub error_code: Option<i32>,
    pub src_tts: Option<String>,
    pub dst_tts: Option<String>,
    pub dict: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Eq, Deserialize)]
pub struct TranslateResult {
    pub src: String,
    pub dst: String,
}
