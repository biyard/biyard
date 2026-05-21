use crate::features::catalog::CategoryScaleResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::{CommonConfig, EntityType, Partition, Result};
#[cfg(feature = "server")]
use crate::features::catalog::models::CategoryScaleAggregate;

#[get("/v1/aggregates/category-scale")]
pub async fn get_category_scale_handler() -> Result<CategoryScaleResponse> {
    let cfg = CommonConfig::default();
    let cli = cfg.dynamodb();

    let pk = Partition::Aggregate.to_string();
    let sk = EntityType::Aggregate("CATEGORY_SCALE".to_string()).to_string();

    let agg = CategoryScaleAggregate::get(cli, pk, Some(sk))
        .await?
        .unwrap_or_default();

    Ok(CategoryScaleResponse {
        music_count: agg.music_count,
        music_amount: agg.music_amount,
        art_count: agg.art_count,
        art_amount: agg.art_amount,
        real_estate_count: agg.real_estate_count,
        real_estate_amount: agg.real_estate_amount,
        livestock_count: agg.livestock_count,
        livestock_amount: agg.livestock_amount,
        total_count: agg.total_count,
        total_amount: agg.total_amount,
        updated_at: agg.updated_at,
    })
}
