use super::{append_query, render_path, require_path, Result, TokenProvider};
use crate::openapi::models::{PanelsQuery, PanelsResponse};

impl<P> super::MenuApi<P>
where
    P: TokenProvider,
{
    /// 分页获取指定生效场景下的指令面板列表。
    ///
    /// `scope` 为必填参数，可选值为 `c2c`、`group`、`channel` 或 `dm`。
    pub async fn panels(
        &self,
        scope: &str,
        cursor: Option<&str>,
        limit: Option<u32>,
    ) -> Result<(http::StatusCode, PanelsResponse)> {
        let template = require_path(&self.paths.panels_get, "panels_get")?;
        let path = render_path(&template, &[])?;
        let path = append_query(
            path,
            &[
                ("scope", Some(scope.to_string())),
                ("cursor", cursor.map(str::to_string)),
                ("limit", limit.map(|value| value.to_string())),
            ],
        );
        self.client.get_t(&path).await
    }

    /// 使用强类型查询参数分页获取指令面板列表。
    pub async fn panels_by_query(
        &self,
        query: &PanelsQuery,
    ) -> Result<(http::StatusCode, PanelsResponse)> {
        self.panels(&query.scope, query.cursor.as_deref(), query.limit)
            .await
    }
}
