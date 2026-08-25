use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest::Method;

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::GroupApi;

impl GroupApi {
    pub async fn get_group_members(
        &self,
        group_openid: &str,
        query: Option<&super::models::group_members_query::GroupMembersQuery>,
    ) -> Result<super::models::group_members_response::GroupMembersResponse, ApiRequestError<ErrResp>>
    {
        let group_openid = utf8_percent_encode(group_openid, NON_ALPHANUMERIC);
        let path = format!("v2/groups/{group_openid}/members");
        let mut params = Vec::new();
        if let Some(query) = query {
            if let Some(limit) = query.limit {
                params.push(("limit", limit.to_string()));
            }
            if let Some(start_index) = query.start_index {
                params.push(("start_index", start_index.to_string()));
            }
        }

        self.request
            .request::<
                serde_json::Value,
                super::models::group_members_response::GroupMembersResponse,
                ErrResp,
            >(&path, Method::POST, Some(&params), None)
            .await
    }
}
