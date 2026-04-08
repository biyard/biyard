use crate::common::{ListResponse, ProjectPartition, Result};
use crate::features::projects::SalesLogResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, ProjectViewerAuth};
#[cfg(feature = "server")]
use crate::features::projects::SalesLog;

#[get("/v1/projects/:project_id/sales-logs?limit&bookmark", auth: ProjectViewerAuth)]
pub async fn list_sales_logs_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
    limit: i32,
    bookmark: Option<String>,
) -> Result<ListResponse<SalesLogResponse>> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();

    // `gsi1` uses `TS#<created_at>` as the range key; scanning in
    // reverse order gives the newest sales logs first without any
    // application-side sorting.
    let opt = SalesLog::opt_with_bookmark(bookmark)
        .limit(limit)
        .scan_index_forward(false);

    let (logs, bookmark) = SalesLog::find_by_project(cli, &auth.project.pk, opt).await?;
    let items: Vec<SalesLogResponse> = logs.into_iter().map(Into::into).collect();

    Ok((items, bookmark).into())
}
