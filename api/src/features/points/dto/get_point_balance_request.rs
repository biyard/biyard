use crate::*;

#[derive(Debug, Deserialize, Serialize, Clone, Default, JsonSchema, OperationIo)]
pub struct GetPointBalanceRequest {
    #[schemars(description = "Month in YYYY-MM / YYYY format")]
    pub month: Option<String>,
}
