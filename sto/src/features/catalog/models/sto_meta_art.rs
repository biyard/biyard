use crate::common::*;

/// 미술품 카테고리 메타 — `pk = STO#{uuid}`, `sk = STO_META#ART`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, DynamoEntity, Default)]
#[dynamo(table = "sto")]
pub struct StoMetaArt {
    pub pk: Partition,
    pub sk: EntityType,

    #[serde(default)]
    pub artist: Option<String>,
    #[serde(default)]
    pub artwork_year: Option<String>,
    #[serde(default)]
    pub medium: Option<String>,
    #[serde(default)]
    pub dimensions: Option<String>,

    pub created_at: i64,
    pub updated_at: i64,
}
