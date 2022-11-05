//! 垂直领域翻译支持范围

use std::{fmt::Display, str::FromStr};

/// 垂直领域翻译支持范围
pub enum Domain {
    /// 电子科技领域
    ///
    /// - 中文-->英语
    Electronics,

    /// 金融财经领域
    ///
    /// - 中文-->英语
    /// - 英语-->中文
    Finance,
    /// 水利机械领域
    /// - 中文-->英语
    Mechanics,
    /// 生物医药领域
    /// - 中文-->英语
    /// - 英语-->中文
    Medicine,
    /// 网络文学领域
    /// - 中文-->英语
    Novel,
}

impl Display for Domain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let domain_str = match self {
            Domain::Electronics => "electronics",
            Domain::Finance => "finance",
            Domain::Mechanics => "mechanics",
            Domain::Medicine => "medicine",
            Domain::Novel => "novel",
        };
        write!(f, "{}", domain_str)
    }
}

impl FromStr for Domain {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "electronics" => Ok(Domain::Electronics),
            "finance" => Ok(Domain::Finance),
            "mechanics" => Ok(Domain::Mechanics),
            "medicine" => Ok(Domain::Medicine),
            "novel" => Ok(Domain::Novel),
            _ => Err("暂不支持的垂直领域类型".to_string()),
        }
    }
}
