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
    Path(ProjectPointPathParam { meta_user_id }): Path<ProjectPointPathParam>,
    Query(GetUserBalanceRequest { date }): Query<GetUserBalanceRequest>,
) -> Result<Json<ListResponse<PointBalanceResponse>>> {
    debug!(
        "Getting point balance for user {} in project {:?}",
        meta_user_id, project
    );

    let user_pk = Partition::MetaUser(meta_user_id);
    let pk = CompositePartition(project.pk.clone(), user_pk);

    let sk = EntityType::Month(date.clone());

    // Get user's point balance
    let (balances, bookmark): (Vec<PointBalance>, Option<String>) =
        PointBalance::query_begins_with_sk(&cli, &pk, &sk).await?;

    // Get project-level monthly aggregation
    let (agg_pk, agg_sk) = MonthlyPointAggregation::keys(project.pk.clone().into(), date.clone());
    let aggregation = MonthlyPointAggregation::get(&cli, &agg_pk, Some(agg_sk)).await?;

    let project_total_points = aggregation.as_ref().map(|a| a.supplied_points).unwrap_or(0);

    let monthly_token_supply = 0;

    let items: Vec<PointBalanceResponse> = balances
        .into_iter()
        .map(|b| {
            let mut response: PointBalanceResponse = b.into();
            response.project_total_points = project_total_points;
            response.monthly_token_supply = monthly_token_supply;
            response
        })
        .collect();

    Ok(Json(ListResponse { items, bookmark }))
}
