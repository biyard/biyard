use crate::common::*;

/// 한우/한돈 등 가축투자계약증권 메타 — `pk = STO#{uuid}`, `sk = STO_META#LIVESTOCK`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, DynamoEntity, Default)]
#[dynamo(table = "sto")]
pub struct StoMetaLivestock {
    pub pk: Partition,
    pub sk: EntityType,

    #[serde(default)]
    pub farm_name: Option<String>,
    #[serde(default)]
    pub breed: Option<String>,
    #[serde(default)]
    pub head_count: Option<i32>,

    pub created_at: i64,
    pub updated_at: i64,
}
