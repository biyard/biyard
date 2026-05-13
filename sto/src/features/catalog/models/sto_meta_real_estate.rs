use crate::common::*;

/// 부동산 카테고리 메타 — `pk = STO#{uuid}`, `sk = STO_META#REAL_ESTATE`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, DynamoEntity, Default)]
#[dynamo(table = "sto")]
pub struct StoMetaRealEstate {
    pub pk: Partition,
    pub sk: EntityType,

    #[serde(default)]
    pub address: Option<String>,
    #[serde(default)]
    pub building_type: Option<String>,
    #[serde(default)]
    pub floor_area: Option<String>,

    pub created_at: i64,
    pub updated_at: i64,
}
