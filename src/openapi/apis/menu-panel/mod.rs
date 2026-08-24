pub(super) use super::Method;
pub(super) use super::{
    append_query, render_path, require_path, OpenApiClient, OpenApiPaths, Result, TokenProvider,
};

mod menu_get;
mod menu_put;
mod panels_get;
pub mod panels_panel_id_delete;
mod panels_panel_id_get;
mod panels_panel_id_put;
pub mod panels_panel_id_target_put;
pub mod panels_post;

pub use menu_get::MenuApi;
