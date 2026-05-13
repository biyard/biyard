use crate::common::*;
use crate::features::catalog::{
    IssuanceStructureDto, OfferingDto, SourceRefDto, StoDetailResponse, StoSummary,
};
use super::StoMetaBundle;

/// STO 공통 row — `pk = STO#{uuid}`, `sk = STO`.
/// 카테고리별 부가 정보(작가·신탁계약 번호·농장명 등)는 동일 PK 에 별도 `sk = STO_META#{CATEGORY}` row 로 분리.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, DynamoEntity, Default)]
#[dynamo(table = "sto")]
pub struct Sto {
    pub pk: Partition,

    #[dynamo(index = "gsi4", pk, name = "find_all")]
    pub sk: EntityType,

    pub name: String,

    #[serde(default)]
    pub underlying: Option<String>,

    pub category: Category,
    pub country: Country,

    #[dynamo(index = "gsi3", pk, prefix = "ISSUER", name = "find_by_issuer_id")]
    #[serde(default)]
    pub issuer_id: String,

    #[serde(default)]
    pub security_type: Option<String>,

    #[serde(default)]
    pub classification: Option<String>,

    #[dynamo(index = "gsi1", pk, prefix = "STATUS", name = "find_by_status")]
    pub status: StoStatus,

    /// 발행/신고 일시 (Unix epoch ms). GSI sort key 로도 활용.
    #[dynamo(index = "gsi1", sk, prefix = "TS")]
    #[dynamo(index = "gsi2", sk, prefix = "TS")]
    #[dynamo(index = "gsi3", sk, prefix = "TS")]
    #[dynamo(index = "gsi4", sk, prefix = "TS")]
    pub issued_at: i64,

    /// `CAT#{country}#{category}` 형태의 GSI2 partition key. 모델에 두는 이유는
    /// DynamoEntity 매크로가 GSI 컬럼 prefix 를 단일 필드로만 지원해서, 두 enum 의
    /// 결합 키를 별도로 유지해야 하기 때문. 시드/저장 시 자동 채움.
    #[dynamo(index = "gsi2", pk, prefix = "CAT", name = "find_by_country_category")]
    #[serde(default)]
    pub country_category: String,

    pub origin: Origin,

    #[serde(default)]
    pub external_id: Option<String>,

    #[serde(default)]
    pub external_url: Option<String>,

    #[serde(default)]
    pub offering: Option<StoOffering>,

    #[serde(default)]
    pub issuance_structure: Option<StoIssuanceStructure>,

    #[serde(default)]
    pub sources: Vec<StoSourceRef>,

    pub created_at: i64,
    pub updated_at: i64,

    #[serde(default)]
    pub sto_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StoOffering {
    #[serde(default)]
    pub amount: Option<i64>,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub unit_price: Option<i64>,
    #[serde(default)]
    pub total_units: Option<i64>,
    #[serde(default)]
    pub subscription_start: Option<String>,
    #[serde(default)]
    pub subscription_end: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StoIssuanceStructure {
    #[serde(default)]
    pub issuer: Option<String>,
    #[serde(default)]
    pub trustee: Option<String>,
    #[serde(default)]
    pub trustee_role: Option<String>,
    #[serde(default)]
    pub underwriter: Option<String>,
    #[serde(default)]
    pub custody: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoSourceRef {
    pub src: String,
    pub label: String,
}

impl Sto {
    pub fn id(&self) -> String {
        match &self.pk {
            Partition::Sto(id) => id.clone(),
            _ => String::new(),
        }
    }
}

impl From<Sto> for StoSummary {
    fn from(s: Sto) -> Self {
        let id = s.id();
        Self {
            sto_id: if id.is_empty() { s.sto_id } else { id },
            name: s.name,
            underlying: s.underlying,
            category: s.category,
            country: s.country,
            issuer_id: if s.issuer_id.is_empty() {
                None
            } else {
                Some(s.issuer_id)
            },
            security_type: s.security_type,
            classification: s.classification,
            status: s.status,
            issued_at: s.issued_at,
            origin: s.origin,
            external_url: s.external_url,
            artist: None,
            rights_category: None,
        }
    }
}

impl Sto {
    pub fn into_detail(
        self,
        meta: StoMetaBundle,
        filings: Vec<crate::features::catalog::FilingSummary>,
    ) -> StoDetailResponse {
        let id = self.id();
        let (artist, rights_category, trust_no, year) = meta
            .music
            .as_ref()
            .map(|m| {
                (
                    m.artist.clone(),
                    m.rights_category.clone(),
                    m.trust_no.clone(),
                    m.year.clone(),
                )
            })
            .unwrap_or((None, None, None, None));

        StoDetailResponse {
            sto_id: if id.is_empty() {
                self.sto_id.clone()
            } else {
                id
            },
            name: self.name,
            underlying: self.underlying,
            category: self.category,
            country: self.country,
            issuer_id: if self.issuer_id.is_empty() {
                None
            } else {
                Some(self.issuer_id)
            },
            security_type: self.security_type,
            classification: self.classification,
            status: self.status,
            issued_at: self.issued_at,
            origin: self.origin,
            external_id: self.external_id,
            external_url: self.external_url,
            artist,
            rights_category,
            trust_no,
            year,
            offering: self.offering.map(|o| OfferingDto {
                amount: o.amount,
                currency: o.currency,
                unit_price: o.unit_price,
                total_units: o.total_units,
                subscription_start: o.subscription_start,
                subscription_end: o.subscription_end,
            }),
            issuance_structure: self.issuance_structure.map(|i| IssuanceStructureDto {
                issuer: i.issuer,
                trustee: i.trustee,
                trustee_role: i.trustee_role,
                underwriter: i.underwriter,
                custody: i.custody,
            }),
            sources: self
                .sources
                .into_iter()
                .map(|s| SourceRefDto {
                    src: s.src,
                    label: s.label,
                })
                .collect(),
            filings,
        }
    }
}

