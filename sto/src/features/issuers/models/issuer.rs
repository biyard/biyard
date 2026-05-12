use crate::common::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, DynamoEntity, Default)]
pub struct Issuer {
    pub pk: Partition,
    pub sk: EntityType,

    pub issuer_id: String,
    pub name: String,
    pub region: String,
    pub country: String,
    pub category: String,
    pub description: String,
    pub status: String,

    #[serde(default)]
    pub sandbox: Option<String>,

    #[serde(default)]
    pub chain: Option<String>,

    #[serde(default)]
    pub website: Option<String>,

    #[serde(default)]
    pub sources: Vec<IssuerSourceRef>,

    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IssuerSourceRef {
    pub src: String,
    pub label: String,
}
