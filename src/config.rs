//! 客户端配置
use crate::lang::Lang;

/// 客户端配置
pub struct Config {
    /// APP ID
    pub app_id: String,
    /// 密钥
    pub secret_key: String,
    /// 源语言，默认auto
    pub from: Lang,
    /// 目标语言，默认auto
    pub to: Lang,
    /// 是否开通词典
    pub open_dict: bool,
    /// 是否开通了TTS
    pub open_tts: bool,
    /// 是否开通了"我的术语"
    pub open_action: bool,
}

impl Config {
    /// create Config
    pub fn new(app_id: String, app_secret: String) -> Self {
        Self {
            app_id,
            secret_key: app_secret,
            from: Lang::default(),
            to: Lang::default(),
            open_dict: false,
            open_tts: false,
            open_action: false,
        }
    }

    /// 设置源语言
    pub fn set_from(&mut self, from: Lang) {
        self.from = from;
    }

    /// 设置目标语言
    pub fn set_to(&mut self, to: Lang) {
        self.to = to;
    }
}
