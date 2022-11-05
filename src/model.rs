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

#[derive(Debug, Clone, PartialEq, Default, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageResult {
    #[serde(rename = "error_code")]
    pub error_code: String,
    #[serde(rename = "error_msg")]
    pub error_msg: String,
    pub data: Option<Data>,
}

#[derive(Debug, Clone, PartialEq, Default, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub from: String,
    pub to: String,
    pub content: Vec<Content>,
    pub sum_src: String,
    pub sum_dst: String,
    pub paste_img: String,
}

#[derive(Debug, Clone, PartialEq, Default, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub src: String,
    pub dst: String,
    pub rect: String,
    pub line_count: i64,
    pub points: Vec<Point>,
    pub paste_img: String,
}

#[derive(Debug, Clone, PartialEq, Default, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Point {
    pub x: i64,
    pub y: i64,
}
