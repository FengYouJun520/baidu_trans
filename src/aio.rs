//! 异步API
use std::cell::RefCell;

use crate::{config::Config, constant::COMMON_URL, lang::Lang, model::common::CommonResult, util};

/// 百度翻译客户端
pub struct Client {
    pub(crate) config: RefCell<Config>,
    pub(crate) http_client: reqwest::Client,
}

impl Client {
    /// 创建客户端
    pub fn new(config: Config) -> Self {
        Self {
            config: RefCell::new(config),
            http_client: reqwest::Client::builder().build().unwrap(),
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
    pub async fn translate(&self, q: &str) -> anyhow::Result<CommonResult> {
        let params = util::build_form(&self.config.borrow(), q);

        let resp = self
            .http_client
            .post(COMMON_URL)
            .form(&params)
            .send()
            .await?;

        Ok(resp.json().await?)
    }

    /// 图片翻译
    #[cfg(feature = "image")]
    pub async fn image_translate(
        &self,
        name: &str,
        data: Vec<u8>,
    ) -> anyhow::Result<crate::model::image::ImageResult> {
        let multipart_form = util::create_image_form(
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
            .send()
            .await?;

        Ok(resp.json().await?)
    }

    /// 垂直领域翻译
    /// - q: 要翻译的文本
    /// - domain: 选择垂直领域
    #[cfg(feature = "domain")]
    pub async fn domain_translate(
        &self,
        q: &str,
        domain: crate::domain::Domain,
    ) -> anyhow::Result<crate::model::domain::DomainResult> {
        use crate::constant::DOMAIN_URL;

        let params = util::build_domain_form(&self.config.borrow(), q, domain);

        let resp = self
            .http_client
            .post(DOMAIN_URL)
            .form(&params)
            .send()
            .await?;

        Ok(resp.json().await?)
    }

    /// 文档翻译统计校验服务
    /// - `path`: 文档文件的路径
    #[cfg(feature = "doc")]
    pub async fn doc_count_translate<P: AsRef<std::path::Path>>(
        &self,
        path: P,
    ) -> anyhow::Result<crate::document::model::DocCountResult> {
        use crate::constant::DOC_COUNT_URL;

        let params = util::build_doc_count_form_aio(&self.config.borrow(), path)?;

        let resp = self
            .http_client
            .post(DOC_COUNT_URL)
            .multipart(params)
            .send()
            .await?;

        Ok(resp.json().await?)
    }
}
