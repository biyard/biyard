use crate::common::*;
use crate::features::catalog::{FilingAttachmentDto, FilingSummary};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, DynamoEntity, Default)]
#[dynamo(table = "sto")]
pub struct Filing {
    pub pk: Partition,
    pub sk: EntityType,

    pub filing_id: String,
    pub filing_source: Origin,

    #[serde(default)]
    pub filing_type: Option<FilingType>,

    pub title: String,
    /// Unix epoch ms — 공시일.
    pub filed_at: i64,

    #[serde(default)]
    pub url: Option<String>,

    #[serde(default)]
    pub attachments: Vec<FilingAttachment>,

    #[serde(default)]
    pub rcept_no: Option<String>,

    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FilingAttachment {
    pub name: String,
    pub url: String,
    #[serde(default)]
    pub size_bytes: Option<i64>,
}

impl From<Filing> for FilingSummary {
    fn from(f: Filing) -> Self {
        Self {
            filing_id: f.filing_id,
            filing_source: f.filing_source,
            filing_type: f.filing_type,
            title: f.title,
            filed_at: f.filed_at,
            url: f.url,
            attachments: f
                .attachments
                .into_iter()
                .map(|a| FilingAttachmentDto {
                    name: a.name,
                    url: a.url,
                    size_bytes: a.size_bytes,
                })
                .collect(),
            rcept_no: f.rcept_no,
        }
    }
}
