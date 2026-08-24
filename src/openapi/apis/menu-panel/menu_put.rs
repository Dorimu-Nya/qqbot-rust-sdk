use super::{render_path, require_path, Method, Result, TokenProvider};
use crate::openapi::models::{MenuPutRequest, MenuPutResponse};

impl<P> super::MenuApi<P>
where
    P: TokenProvider,
{
    /// 修改全局自定义菜单。
    pub async fn put(&self, body: &MenuPutRequest) -> Result<(http::StatusCode, MenuPutResponse)> {
        let template = require_path(&self.paths.menu_put, "menu_put")?;
        let path = render_path(&template, &[])?;
        self.client
            .request_t_with(Method::PUT, &path, Some(body))
            .await
    }
}
