use super::{render_path, require_path, Method, Result, TokenProvider};
use crate::openapi::models::{CreatePanelRequest, CreatePanelResponse};

impl<P> super::MenuApi<P>
where
    P: TokenProvider,
{
    /// 创建指令面板。
    pub async fn create(
        &self,
        body: &CreatePanelRequest,
    ) -> Result<(http::StatusCode, CreatePanelResponse)> {
        let template = require_path(&self.paths.panels_create, "panels_create")?;
        let path = render_path(&template, &[])?;
        self.client
            .request_t_with(Method::POST, &path, Some(body))
            .await
    }

    /// 创建指令面板（`create` 的兼容性别名）。
    pub async fn post(
        &self,
        body: &CreatePanelRequest,
    ) -> Result<(http::StatusCode, CreatePanelResponse)> {
        self.create(body).await
    }
}
