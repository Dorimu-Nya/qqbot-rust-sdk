use super::{render_path, require_path, Method, Result, TokenProvider};
use crate::openapi::models::{UpdatePanelRequest, UpdatePanelResponse, UpdatePanelTargetRequest};

impl<P> super::MenuApi<P>
where
    P: TokenProvider,
{
    /// 修改指定指令面板的配置内容。
    pub async fn update_panel(
        &self,
        panel_id: &str,
        body: &UpdatePanelRequest,
    ) -> Result<(http::StatusCode, UpdatePanelResponse)> {
        let template = require_path(&self.paths.panels_panel_id_put, "panels_panel_id_put")?;
        let path = render_path(&template, &[("panel_id", panel_id)])?;
        self.client
            .request_t_with(Method::PUT, &path, Some(body))
            .await
    }

    /// 修改指定指令面板的配置（`update_panel` 的兼容性别名）。
    pub async fn update(
        &self,
        panel_id: &str,
        body: &UpdatePanelRequest,
    ) -> Result<(http::StatusCode, UpdatePanelResponse)> {
        self.update_panel(panel_id, body).await
    }

    /// 修改指定指令面板关联的用户或群。
    pub async fn update_panel_target(
        &self,
        panel_id: &str,
        body: &UpdatePanelTargetRequest,
    ) -> Result<http::StatusCode> {
        let template = require_path(
            &self.paths.panels_panel_id_target_put,
            "panels_panel_id_target_put",
        )?;
        let path = render_path(&template, &[("panel_id", panel_id)])?;
        let response = self
            .client
            .request_json_with(Method::PUT, &path, Some(body))
            .await?;
        Ok(response.status())
    }

    /// 修改指定指令面板关联对象（`update_panel_target` 的兼容性别名）。
    pub async fn update_target(
        &self,
        panel_id: &str,
        body: &UpdatePanelTargetRequest,
    ) -> Result<http::StatusCode> {
        self.update_panel_target(panel_id, body).await
    }
}
