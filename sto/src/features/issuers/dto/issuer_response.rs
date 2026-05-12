use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IssuerSummary {
    pub issuer_id: String,
    pub name: String,
    pub region: String,
    pub country: String,
    pub category: String,
    pub description: String,
    pub status: String,
    pub sandbox: Option<String>,
    pub chain: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IssuerDetailResponse {
    pub issuer: IssuerSummary,
    pub stos: Vec<crate::features::catalog::StoSummary>,
}
