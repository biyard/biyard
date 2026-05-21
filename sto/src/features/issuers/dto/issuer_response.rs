use serde::{Deserialize, Serialize};

use crate::common::{Category, Country, IssuerStatus};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IssuerSummary {
    pub issuer_id: String,
    pub name: String,
    pub country: Country,
    pub category: Category,
    pub description: String,
    pub status: IssuerStatus,
    /// 시리즈/인가 등 자유 코멘트. 상태(`status`) 와 분리.
    pub status_note: Option<String>,
    pub sandbox: Option<String>,
    pub chain: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IssuerDetailResponse {
    pub issuer: IssuerSummary,
    pub stos: Vec<crate::features::catalog::StoSummary>,
}
