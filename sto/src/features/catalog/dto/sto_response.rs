use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoSummary {
    pub sto_id: String,
    pub name: String,
    pub category: String,
    pub region: String,
    pub country: String,
    pub issuer_id: Option<String>,
    pub security_type: Option<String>,
    pub classification: Option<String>,
    pub status: String,
    pub issued_at: String,
    pub origin: String,
    pub external_url: Option<String>,
    pub artist: Option<String>,
    pub rights_category: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoListResponse {
    pub items: Vec<StoSummary>,
    pub total: usize,
}
