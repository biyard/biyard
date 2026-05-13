use crate::common::*;

/// 음악 IP 카테고리 메타 — `pk = STO#{uuid}`, `sk = STO_META#MUSIC`.
/// 음악 STO 한정으로 별도 row 에 저장. Query 시 sk 분기로 채움.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, DynamoEntity, Default)]
#[dynamo(table = "sto")]
pub struct StoMetaMusic {
    pub pk: Partition,
    pub sk: EntityType,

    #[serde(default)]
    pub artist: Option<String>,
    #[serde(default)]
    pub rights_category: Option<String>,
    #[serde(default)]
    pub trust_no: Option<String>,
    #[serde(default)]
    pub year: Option<String>,

    pub created_at: i64,
    pub updated_at: i64,
}
