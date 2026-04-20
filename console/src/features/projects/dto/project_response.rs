use crate::common::types::Partition;
use crate::features::projects::ProjectStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[derive(api_doc_macros::ApiDocSchema)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub struct ProjectResponse {
    /// Unique project identifier.
    #[field_doc(en = "Unique project identifier.", ko = "프로젝트 고유 식별자.")]
    pub id: String,
    /// Owner account identifier.
    #[field_doc(en = "Owner account identifier.", ko = "소유자 계정 식별자.")]
    #[cfg_attr(feature = "server", schemars(with = "String"))]
    pub account_id: Partition,
    /// Enterprise (organization) identifier.
    #[field_doc(en = "Enterprise (organization) identifier.", ko = "기업(조직) 식별자.")]
    #[serde(default)]
    #[cfg_attr(feature = "server", schemars(with = "String"))]
    pub organization_id: Partition,
    /// Project display name.
    #[field_doc(en = "Project display name.", ko = "프로젝트 표시 이름.")]
    pub name: String,
    /// Project description.
    #[field_doc(en = "Project description.", ko = "프로젝트 설명.")]
    pub description: Option<String>,
    /// Brand logo image URL.
    #[field_doc(en = "Brand logo image URL.", ko = "브랜드 로고 이미지 URL.")]
    pub brand_logo_url: Option<String>,
    /// Monthly token supply for this project.
    #[field_doc(en = "Monthly token supply for this project.", ko = "프로젝트의 월간 토큰 공급량.")]
    pub monthly_token_supply: i64,
    /// Treasury reserve rate (e.g. 0.2 = 20%).
    #[field_doc(en = "Treasury reserve rate (e.g. 0.2 = 20%).", ko = "트레저리 준비율 (예: 0.2 = 20%).")]
    pub treasury_reserve_rate: f64,
    /// Project status: Active or Inactive.
    #[field_doc(en = "Project status: Active or Inactive.", ko = "프로젝트 상태: 활성 또는 비활성.")]
    #[cfg_attr(feature = "server", schemars(with = "String"))]
    pub status: ProjectStatus,
    /// Creation timestamp (Unix epoch seconds).
    #[field_doc(en = "Creation timestamp (Unix epoch seconds).", ko = "생성 타임스탬프 (Unix epoch 초).")]
    pub created_at: i64,
    /// Last update timestamp (Unix epoch seconds).
    #[field_doc(en = "Last update timestamp (Unix epoch seconds).", ko = "마지막 업데이트 타임스탬프 (Unix epoch 초).")]
    pub updated_at: i64,
}
