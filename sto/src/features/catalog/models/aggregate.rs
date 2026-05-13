use crate::common::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, DynamoEntity, Default)]
#[dynamo(table = "sto")]
pub struct CategoryScaleAggregate {
    pub pk: Partition,
    pub sk: EntityType,

    #[serde(default)]
    pub music_count: i64,
    #[serde(default)]
    pub music_amount: i64,

    #[serde(default)]
    pub art_count: i64,
    #[serde(default)]
    pub art_amount: i64,

    #[serde(default)]
    pub real_estate_count: i64,
    #[serde(default)]
    pub real_estate_amount: i64,

    #[serde(default)]
    pub livestock_count: i64,
    #[serde(default)]
    pub livestock_amount: i64,

    #[serde(default)]
    pub total_count: i64,
    #[serde(default)]
    pub total_amount: i64,

    pub updated_at: i64,
}
