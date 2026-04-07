use crate::common::{ProjectPartition, Result};
use crate::features::projects::SalesLogResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, ProjectAdminAuth};
#[cfg(feature = "server")]
use crate::features::projects::SalesLog;

#[post("/v1/projects/:project_id/sales-logs", auth: ProjectAdminAuth)]
pub async fn create_sales_log_handler(
    #[allow(unused_variables)] project_id: ProjectPartition,
    amount: i64,
    memo: Option<String>,
) -> Result<SalesLogResponse> {
    let config = CommonConfig::default();
    let cli = config.dynamodb();

    let amount = amount.max(0);
    let memo = memo.and_then(|m| {
        let trimmed = m.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    });

    let log = SalesLog::new(auth.project.pk, amount, memo);
    log.create(cli).await?;

    Ok(log.into())
}
