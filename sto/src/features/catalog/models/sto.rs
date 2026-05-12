//! Sto 엔티티 — server/web 양쪽에서 사용.
//! - server: DynamoDB scan/query 결과를 serde_dynamo 로 역직렬화
//! - web: server 함수 응답 / 컴포넌트 props 로 사용
//! 1차는 raw serde 매핑. 후속에 console 의 Partition/EntityType + DynamoEntity 매크로로 확장.

use serde::{Deserialize, Serialize};

use crate::features::catalog::StoSummary;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Sto {
    pub pk: String,
    pub sk: String,
    pub sto_id: String,
    pub name: String,
    #[serde(default)]
    pub underlying: Option<String>,
    pub category: String,
    pub region: String,
    pub country: String,
    #[serde(default)]
    pub issuer_id: Option<String>,
    #[serde(default)]
    pub security_type: Option<String>,
    #[serde(default)]
    pub classification: Option<String>,
    pub status: String,
    pub issued_at: String,
    pub origin: String,
    #[serde(default)]
    pub external_id: Option<String>,
    #[serde(default)]
    pub external_url: Option<String>,
    #[serde(default)]
    pub artist: Option<String>,
    #[serde(default)]
    pub rights_category: Option<String>,
    #[serde(default)]
    pub trust_no: Option<String>,
    #[serde(default)]
    pub year: Option<String>,
    #[serde(default)]
    pub created_at: i64,
    #[serde(default)]
    pub updated_at: i64,
}

impl From<Sto> for StoSummary {
    fn from(s: Sto) -> Self {
        Self {
            sto_id: s.sto_id,
            name: s.name,
            category: s.category,
            region: s.region,
            country: s.country,
            issuer_id: s.issuer_id,
            security_type: s.security_type,
            classification: s.classification,
            status: s.status,
            issued_at: s.issued_at,
            origin: s.origin,
            external_url: s.external_url,
            artist: s.artist,
            rights_category: s.rights_category,
        }
    }
}
