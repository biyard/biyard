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
    #[serde(default)]
    pub land_area: Option<String>,
    #[serde(default)]
    pub floors: Option<String>,
    #[serde(default)]
    pub completion_date: Option<String>,
    #[serde(default)]
    pub trustee: Option<String>,
    #[serde(default)]
    pub tenant: Option<String>,
    #[serde(default)]
    pub lease_term: Option<String>,
    #[serde(default)]
    pub total_offering: Option<String>,
    #[serde(default)]
    pub total_units: Option<String>,
    #[serde(default)]
    pub unit_price: Option<String>,
    #[serde(default)]
    pub upfront_fee: Option<String>,
    #[serde(default)]
    pub dividend_frequency: Option<String>,
    #[serde(default)]
    pub appraisal_values: Option<serde_json::Value>,

    pub created_at: i64,
    pub updated_at: i64,
}
