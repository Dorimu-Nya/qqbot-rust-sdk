use super::{render_path, require_path, OpenApiClient, OpenApiPaths, Result, TokenProvider};
use crate::openapi::models::MenuResponse;

/// 菜单面板相关接口。
#[derive(Clone)]
pub struct MenuApi<P> {
    pub(in crate::openapi::apis) client: OpenApiClient<P>,
    pub(in crate::openapi::apis) paths: OpenApiPaths,
}

impl<P> MenuApi<P>
where
    P: TokenProvider,
{
    /// 获取当前生效的菜单配置。
    pub async fn get(&self) -> Result<(http::StatusCode, MenuResponse)> {
        let template = require_path(&self.paths.menu_get, "menu_get")?;
        let path = render_path(&template, &[])?;
        self.client.get_t(&path).await
    }
}
