use crate::common::*;
use crate::features::catalog::{
    IssuanceStructureDto, OfferingDto, SourceRefDto, StoDetailResponse, StoSummary,
};

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

    #[dynamo(index = "gsi1", sk)]
    #[dynamo(index = "gsi2", sk)]
    #[dynamo(index = "gsi3", sk)]
    pub issued_at: String,

    #[dynamo(index = "gsi2", pk, prefix = "CAT", name = "find_by_region_category")]
    #[serde(default)]
    pub region_category: String,

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
            artist: s.artist,
            rights_category: s.rights_category,
        }
    }
}

impl Sto {
    pub fn into_detail(
        self,
        filings: Vec<crate::features::catalog::FilingSummary>,
    ) -> StoDetailResponse {
        let id = self.id();
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
            artist: self.artist,
            rights_category: self.rights_category,
            trust_no: self.trust_no,
            year: self.year,
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
