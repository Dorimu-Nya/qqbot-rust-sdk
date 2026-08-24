use super::{render_path, require_path, Method, Result, TokenProvider};

impl<P> super::MenuApi<P>
where
    P: TokenProvider,
{
    /// 删除指定的指令面板。
    pub async fn delete(&self, panel_id: &str) -> Result<http::StatusCode> {
        let template = require_path(&self.paths.panels_panel_id_delete, "panels_panel_id_delete")?;
        let path = render_path(&template, &[("panel_id", panel_id)])?;
        let response = self
            .client
            .request_json(Method::DELETE, &path, None)
            .await?;
        Ok(response.status())
    }

    /// 删除指定的指令面板（`delete` 的兼容性别名）。
    pub async fn delete_panel(&self, panel_id: &str) -> Result<http::StatusCode> {
        self.delete(panel_id).await
    }
}
