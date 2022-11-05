use std::cell::RefCell;

use crate::{config::Config, constant::COMMON_URL, lang::Lang, model::CommonResult, util};

/// 百度翻译客户端
pub struct Client {
    pub(crate) config: RefCell<Config>,
    pub(crate) http_client: reqwest::blocking::Client,
}

impl Client {
    pub fn new(config: Config) -> Self {
        Self {
            config: RefCell::new(config),
            http_client: reqwest::blocking::Client::builder().build().unwrap(),
        }
    }

    /// 设置源语言和目标语言
    pub fn lang(&self, from: Lang, to: Lang) {
        self.config.borrow_mut().set_from(from);
        self.config.borrow_mut().set_to(to);
    }
}

impl Client {
    /// 通用翻译
    /// - q: 源文本
    #[cfg(feature = "blocking")]
    pub fn translate(&self, q: &str) -> anyhow::Result<CommonResult> {
        let params = util::build_form(&self.config.borrow(), q);

        let resp = self.http_client.post(COMMON_URL).form(&params).send()?;

        Ok(resp.json()?)
    }

    /// 图片翻译
    /// - name: 文件名
    /// - data: 图片数据
    #[cfg(feature = "image")]
    pub fn image_translate(
        &self,
        name: &str,
        data: Vec<u8>,
    ) -> anyhow::Result<crate::model::ImageResult> {
        let multipart_form = util::create_image_form_blocking(
            name,
            data,
            &self.config.borrow(),
            "APICUID",
            "mac",
            "3",
            None,
        );
        let resp = self
            .http_client
            .post(crate::constant::IMAGE_URL)
            .multipart(multipart_form)
            .send()?;

        Ok(resp.json()?)
    }
}
