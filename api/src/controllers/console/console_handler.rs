use tmpl_renderer::{BootData, PageMeta};

use crate::axum::response::{Html, IntoResponse};
use crate::{AppState, axum::extract::State};

pub async fn console_handler(
    State(_app_state): State<AppState>,
    (page, tmpl): super::ConsolePageTmpl,
) -> Result<impl IntoResponse, crate::Error> {
    let boot = BootData::new();

    let template = tmpl
        .with_title(page.title())
        .with_description(page.description())
        .with_boot_json(boot.to_json::<crate::Error>()?);

    Ok(Html(template.to_html()?))
}
