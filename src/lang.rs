use std::fmt::Display;

/// 常见语种
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Lang {
    /// 自动
    #[default]
    Auto,
    /// 中文
    Zh,
    /// 英语
    En,
    /// 粤语
    Yue,
    /// 文言文
    Wyw,
    /// 日语
    Jp,
    /// 韩语
    Kor,
    /// 法语
    Fra,
    /// 西班牙语
    Spa,
    /// 泰语
    Th,
    /// 阿拉伯语
    Ara,
    /// 俄语
    Ru,
    /// 葡萄牙语
    Pt,
    /// 德语
    De,
    /// 意大利语
    It,
    /// 希腊语
    El,
    /// 荷兰语
    Nl,
    /// 波兰语
    Pl,
    /// 保加利亚语
    Bul,
    /// 爱沙尼亚语
    Est,
    /// 丹麦语
    Dan,
    /// 芬兰语
    Fin,
    /// 捷克语
    Cs,
    /// 罗马尼亚语
    Rom,
    /// 斯洛文尼亚语
    Slo,
    /// 瑞典语
    Swe,
    /// 匈牙利语
    Hu,
    /// 繁体中文
    Cht,
    /// 越南语
    Vie,
}

impl Display for Lang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lang = match self {
            Lang::Auto => "auto",
            Lang::Zh => "zh",
            Lang::En => "en",
            Lang::Yue => "yue",
            Lang::Wyw => "wyw",
            Lang::Jp => "jp",
            Lang::Kor => "kor",
            Lang::Fra => "fra",
            Lang::Spa => "spa",
            Lang::Th => "th",
            Lang::Ara => "ara",
            Lang::Ru => "ru",
            Lang::Pt => "pt",
            Lang::De => "de",
            Lang::It => "it",
            Lang::El => "el",
            Lang::Nl => "nl",
            Lang::Pl => "pl",
            Lang::Bul => "bul",
            Lang::Est => "est",
            Lang::Dan => "dan",
            Lang::Fin => "fin",
            Lang::Cs => "cs",
            Lang::Rom => "rom",
            Lang::Slo => "slo",
            Lang::Swe => "swe",
            Lang::Hu => "hu",
            Lang::Cht => "cht",
            Lang::Vie => "vie",
        };

        write!(f, "{}", lang)
    }
}
