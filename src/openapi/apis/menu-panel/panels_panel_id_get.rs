use super::{render_path, require_path, Result, TokenProvider};
use crate::openapi::models::PanelDetailResponse;

impl<P> super::MenuApi<P>
where
    P: TokenProvider,
{
    /// 查询指定指令面板的完整配置详情。
    pub async fn get_panel(
        &self,
        panel_id: &str,
    ) -> Result<(http::StatusCode, PanelDetailResponse)> {
        let template = require_path(&self.paths.panels_panel_id_get, "panels_panel_id_get")?;
        let path = render_path(&template, &[("panel_id", panel_id)])?;
        self.client.get_t(&path).await
    }

    /// 查询指定指令面板详情（`get_panel` 的兼容性别名）。
    pub async fn panel(&self, panel_id: &str) -> Result<(http::StatusCode, PanelDetailResponse)> {
        self.get_panel(panel_id).await
    }
}
