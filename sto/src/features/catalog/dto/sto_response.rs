use serde::{Deserialize, Serialize};

use crate::common::{Category, Country, Origin, StoStatus};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoSummary {
    pub sto_id: String,
    pub name: String,
    pub underlying: Option<String>,
    pub category: Category,
    pub country: Country,
    pub issuer_id: Option<String>,
    pub security_type: Option<String>,
    pub classification: Option<String>,
    pub status: StoStatus,
    /// Unix epoch ms — 발행/신고일.
    pub issued_at: i64,
    pub origin: Origin,
    pub external_url: Option<String>,
    pub artist: Option<String>,
    pub rights_category: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoListResponse {
    pub items: Vec<StoSummary>,
    pub total: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OfferingDto {
    pub amount: Option<i64>,
    pub currency: Option<String>,
    pub unit_price: Option<i64>,
    pub total_units: Option<i64>,
    pub subscription_start: Option<String>,
    pub subscription_end: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct IssuanceStructureDto {
    pub issuer: Option<String>,
    pub trustee: Option<String>,
    pub trustee_role: Option<String>,
    pub underwriter: Option<String>,
    pub custody: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SourceRefDto {
    pub src: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoDetailResponse {
    pub sto_id: String,
    pub name: String,
    pub underlying: Option<String>,
    pub category: Category,
    pub country: Country,
    pub issuer_id: Option<String>,
    pub security_type: Option<String>,
    pub classification: Option<String>,
    pub status: StoStatus,
    /// Unix epoch ms — 발행/신고일.
    pub issued_at: i64,
    pub origin: Origin,
    pub external_id: Option<String>,
    pub external_url: Option<String>,
    pub artist: Option<String>,
    pub rights_category: Option<String>,
    pub trust_no: Option<String>,
    pub year: Option<String>,
    pub offering: Option<OfferingDto>,
    pub issuance_structure: Option<IssuanceStructureDto>,
    pub sources: Vec<SourceRefDto>,
    pub filings: Vec<FilingSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FilingAttachmentDto {
    pub name: String,
    pub url: String,
    pub size_bytes: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CategoryScaleResponse {
    pub music_count: i64,
    pub music_amount: i64,
    pub art_count: i64,
    pub art_amount: i64,
    pub real_estate_count: i64,
    pub real_estate_amount: i64,
    pub livestock_count: i64,
    pub livestock_amount: i64,
    pub total_count: i64,
    pub total_amount: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FilingSummary {
    pub filing_id: String,
    pub filing_source: crate::common::Origin,
    pub filing_type: Option<crate::common::FilingType>,
    pub title: String,
    /// Unix epoch ms — 공시일.
    pub filed_at: i64,
    pub url: Option<String>,
    pub attachments: Vec<FilingAttachmentDto>,
    pub rcept_no: Option<String>,
}
