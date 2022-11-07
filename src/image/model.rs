//! 图片翻译返回的相关结构
use serde::Deserialize;

/// 图片翻译返回的结构
#[derive(Debug, Clone, PartialEq, Default, Eq, Deserialize)]
pub struct ImageResult {
    /// 错误码
    pub error_code: String,
    /// 错误消息
    pub error_msg: String,
    /// 返回数据集合
    pub data: Option<Data>,
}

/// 返回结果
#[derive(Debug, Clone, PartialEq, Default, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    /// 源语种方向
    pub from: String,
    ///目标语种方向
    pub to: String,
    /// 分段内容
    pub content: Vec<Content>,
    /// 未分段翻译原文
    pub sum_src: String,
    /// 未分段翻译译文
    pub sum_dst: String,
    /// 图片贴合 (整屏贴合)，paste=1有效，base64格式
    pub paste_img: String,
}

/// 翻译内容
#[derive(Debug, Clone, PartialEq, Default, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    /// 分段翻译的原文
    pub src: String,
    /// 分段翻译的译文
    pub dst: String,
    /// 表示识别出的文字的位置，坐标为左上角，依次是left、top、width、height
    pub rect: String,
    /// 表示该分段信息是原文的多少行合并在一起
    pub line_count: i64,
    /// 译文矩形坐标
    /// 格式为:
    /// ```ignore
    /// [
    ///   {"x": 254,"y": 280},
    ///   {"x": 506,"y": 278},
    ///   {"x": 506,"y": 303},
    ///   {"x": 254,"y": 305}
    /// ]
    /// ```
    pub points: Vec<Point>,
    /// 图片贴合 (分块贴合)，分段贴合图片，paste=2有效，base64格式
    pub paste_img: String,
}

/// 矩阵坐标
#[derive(Debug, Clone, PartialEq, Default, Eq, Deserialize)]
pub struct Point {
    /// x坐标
    pub x: i64,
    /// y坐标
    pub y: i64,
}
