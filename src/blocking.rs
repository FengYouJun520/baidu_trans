//! 阻塞API
use std::cell::RefCell;

use crate::{config::Config, constant::COMMON_URL, lang::Lang, model::CommonResult, util};

/// 百度翻译客户端
pub struct Client {
    pub(crate) config: RefCell<Config>,
    pub(crate) http_client: reqwest::blocking::Client,
}

impl Client {
    /// 创建客户端
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
    ) -> anyhow::Result<crate::image::model::ImageResult> {
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

    /// 垂直领域翻译
    /// - q: 要翻译的文本
    /// - domain: 选择垂直领域
    #[cfg(feature = "domain")]
    pub fn domain_translate(
        &self,
        q: &str,
        domain: crate::domain::Domain,
    ) -> anyhow::Result<crate::domain::model::DomainResult> {
        use crate::constant::DOMAIN_URL;

        let params = util::build_domain_form(&self.config.borrow(), q, domain);

        let resp = self.http_client.post(DOMAIN_URL).form(&params).send()?;
        Ok(resp.json()?)
    }

    /// 文档翻译统计校验服务
    /// - `data`: 文件数据
    /// - `name`: 文件名
    /// - `ext`: 文件扩展名
    #[cfg(feature = "doc")]
    pub fn doc_count_translate(
        &self,
        data: Vec<u8>,
        name: &str,
        ext: &str,
    ) -> anyhow::Result<crate::document::model::DocCountResult> {
        use crate::constant::DOC_COUNT_URL;

        let params = util::build_doc_count_form_blocking(&self.config.borrow(), data, name, ext)?;

        let resp = self
            .http_client
            .post(DOC_COUNT_URL)
            .multipart(params)
            .send()?;

        Ok(resp.json()?)
    }

    /// 文档翻译服务
    /// - `data`: 文件数据
    /// - `name`: 文件名
    /// - `ext`: 文件扩展名
    /// - `out_type`: 输出扩展名
    #[cfg(feature = "doc")]
    pub fn doc_translate(
        &self,
        data: Vec<u8>,
        name: &str,
        ext: &str,
        out_type: &str,
    ) -> anyhow::Result<crate::document::model::DocResult> {
        use crate::constant::DOC_URL;

        let params =
            util::build_doc_form_blocking(&self.config.borrow(), data, name, ext, out_type)?;

        let resp = self.http_client.post(DOC_URL).multipart(params).send()?;

        Ok(resp.json()?)
    }
}
