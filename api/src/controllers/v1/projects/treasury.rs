use crate::features::points::*;
use crate::features::projects::*;
use crate::*;

pub async fn get_treasury_handler(
    State(AppState { cli, .. }): State<AppState>,
    Extension(project): Extension<Project>,
) -> Result<Json<TreasuryResponse>> {
    debug!("Getting treasury for project {:?}", project.pk);

    let month = time_utils::timestamp_to_yyyy_mm();

    let (agg_pk, agg_sk) =
        MonthlyPointAggregation::keys(project.pk.clone().into(), month.clone());

    let aggregation = MonthlyPointAggregation::get(&cli, &agg_pk, Some(agg_sk))
        .await?
        .unwrap_or_default();

    let total_supply = project.monthly_token_supply;

    // Total treasury is the sum of all points supplied (awarded through purchases and activities)
    let total_treasury = aggregation.supplied_points;

    // Circulating supply = total_supply minus tokens consumed via exchange
    let circulating_supply = (total_supply - aggregation.exchanged_points).max(0);

    // Floor price = treasury / total_supply (guard against division by zero)
    let floor_price = if total_supply > 0 {
        total_treasury as f64 / total_supply as f64
    } else {
        0.0
    };

    Ok(Json(TreasuryResponse {
        total_treasury,
        floor_price,
        total_supply,
        circulating_supply,
    }))
}
