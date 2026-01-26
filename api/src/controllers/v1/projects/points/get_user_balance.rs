use crate::features::points::*;
use crate::features::projects::*;
use crate::features::tokens::*;
use crate::utils::time_utils::timestamp_to_yyyy_mm;
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct GetUserBalanceRequest {
    #[schemars(description = "Date in YYYY-MM format/YYYY format")]
    #[serde(default = "time_utils::timestamp_to_yyyy_mm")]
    pub date: String,
}

pub async fn get_balance_handler(
    State(AppState { cli, .. }): State<AppState>,
    Extension(project): Extension<Project>,
    Path(ProjectUserPathParam { meta_user_id, .. }): ProjectUserPath,
    Query(GetUserBalanceRequest { date }): Query<GetUserBalanceRequest>,
) -> Result<Json<PointBalanceResponse>> {
    debug!(
        "Getting point balance for user {} in project {:?}",
        meta_user_id, project
    );

    let (pb_pk, pb_sk) = PointBalance::keys(
        project.pk.clone().into(),
        meta_user_id.clone(),
        date.clone(),
    );
    let balance = PointBalance::get(&cli, &pb_pk, Some(pb_sk))
        .await?
        .unwrap_or_default();

    let (agg_pk, agg_sk) = MonthlyPointAggregation::keys(project.pk.clone().into(), date.clone());
    let project_total_points = if let Ok(Some(aggregation)) =
        MonthlyPointAggregation::get(&cli, &agg_pk, Some(agg_sk)).await
    {
        aggregation.supplied_points
    } else {
        0
    };
    let monthly_token_supply = project.monthly_token_supply;

    Ok(Json(PointBalanceResponse {
        project_id: project.pk.clone().into(),
        meta_user_id,
        month: date.clone(),
        balance: balance.balance,
        total_earned: balance.total_earned,
        total_spent: balance.total_spent,
        updated_at: balance.updated_at,
        project_total_points,
        monthly_token_supply,
    }))
}
