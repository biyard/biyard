use crate::common::*;
use crate::features::catalog::{
    IssuanceStructureDto, OfferingDto, SourceRefDto, StoDetailResponse, StoSummary,
};

/// STO 공통 row — `pk = STO#{uuid}`, `sk = STO`.
/// 카테고리별 부가 정보(작가, 신탁계약 번호 등)는 [`StoCategoryMeta`] 로 별도 row 에 적재.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, DynamoEntity, Default)]
#[dynamo(table = "sto")]
pub struct Sto {
    pub pk: Partition,
    pub sk: EntityType,

    pub name: String,

    #[serde(default)]
    pub underlying: Option<String>,

    pub category: String,
    pub region: String,
    pub country: String,

    #[dynamo(index = "gsi3", pk, prefix = "ISSUER", name = "find_by_issuer_id")]
    #[serde(default)]
    pub issuer_id: String,

    #[serde(default)]
    pub security_type: Option<String>,

    #[serde(default)]
    pub classification: Option<String>,

    #[dynamo(index = "gsi1", pk, prefix = "STATUS", name = "find_by_status")]
    pub status: String,

    /// 발행/신고 일시 (Unix epoch ms). GSI sort key 로도 활용.
    #[dynamo(index = "gsi1", sk, prefix = "TS")]
    #[dynamo(index = "gsi2", sk, prefix = "TS")]
    #[dynamo(index = "gsi3", sk, prefix = "TS")]
    pub issued_at: i64,

    #[dynamo(index = "gsi2", pk, prefix = "CAT", name = "find_by_region_category")]
    #[serde(default)]
    pub region_category: String,

    pub origin: String,

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
            region: s.region,
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
        meta: Option<StoCategoryMeta>,
        filings: Vec<crate::features::catalog::FilingSummary>,
    ) -> StoDetailResponse {
        let id = self.id();
        let (artist, rights_category, trust_no, year) = match meta {
            Some(StoCategoryMeta::Music {
                artist,
                rights_category,
                trust_no,
                year,
                ..
            }) => (artist, rights_category, trust_no, year),
            _ => (None, None, None, None),
        };

        StoDetailResponse {
            sto_id: if id.is_empty() {
                self.sto_id.clone()
            } else {
                id
            },
            name: self.name,
            underlying: self.underlying,
            category: self.category,
            region: self.region,
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

/// 카테고리별 부가 메타 — 같은 `pk = STO#{uuid}` 에 `sk = STO_META#{CATEGORY}` 로 저장.
/// Query 한 번으로 공통 row + 메타 row + filings 까지 함께 읽음.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, DynamoEntity)]
#[dynamo(table = "sto")]
pub struct StoMetaRow {
    pub pk: Partition,
    pub sk: EntityType,

    pub meta: StoCategoryMeta,

    pub created_at: i64,
    pub updated_at: i64,
}

impl Default for StoMetaRow {
    fn default() -> Self {
        Self {
            pk: Partition::default(),
            sk: EntityType::default(),
            meta: StoCategoryMeta::None,
            created_at: 0,
            updated_at: 0,
        }
    }
}

/// 카테고리별 메타 데이터. JSON 상에서는 `{ "kind": "Music", "artist": ... }` 형태로 직렬화.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(tag = "kind")]
pub enum StoCategoryMeta {
    #[default]
    None,

    /// 음악 IP 신탁수익증권 등
    Music {
        #[serde(default)]
        artist: Option<String>,
        #[serde(default)]
        rights_category: Option<String>,
        #[serde(default)]
        trust_no: Option<String>,
        #[serde(default)]
        year: Option<String>,
    },

    /// 미술품 투자계약증권
    Art {
        #[serde(default)]
        artwork_year: Option<String>,
        #[serde(default)]
        medium: Option<String>,
        #[serde(default)]
        dimensions: Option<String>,
    },

    /// 부동산 수익증권 / DABS
    RealEstate {
        #[serde(default)]
        address: Option<String>,
        #[serde(default)]
        building_type: Option<String>,
        #[serde(default)]
        floor_area: Option<String>,
    },

    /// 한우 등 가축투자계약증권
    Livestock {
        #[serde(default)]
        farm_name: Option<String>,
        #[serde(default)]
        breed: Option<String>,
        #[serde(default)]
        head_count: Option<i32>,
    },
}
