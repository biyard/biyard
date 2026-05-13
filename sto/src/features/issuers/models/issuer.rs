use crate::common::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, DynamoEntity, Default)]
#[dynamo(table = "sto")]
pub struct Issuer {
    pub pk: Partition,

    #[dynamo(index = "gsi4", pk, name = "find_all")]
    pub sk: EntityType,

    pub issuer_id: String,
    pub name: String,
    pub country: Country,
    pub category: Category,
    pub description: String,

    /// 운영/위기/종료 등 상태 enum. 시리즈 단계·인가 결과 같은 자유 코멘트는 `status_note` 로.
    pub status: IssuerStatus,

    #[serde(default)]
    pub status_note: Option<String>,

    #[serde(default)]
    pub sandbox: Option<String>,

    #[serde(default)]
    pub chain: Option<String>,

    #[serde(default)]
    pub website: Option<String>,

    #[serde(default)]
    pub sources: Vec<IssuerSourceRef>,

    #[dynamo(index = "gsi4", sk, prefix = "TS")]
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IssuerSourceRef {
    pub src: String,
    pub label: String,
}
