use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct TreasuryResponse {
    #[schemars(description = "Total treasury value accumulated from purchases")]
    pub total_treasury: i64,

    #[schemars(description = "Floor price derived from treasury / total_supply")]
    pub floor_price: f64,

    #[schemars(description = "Total monthly token supply for the project")]
    pub total_supply: i64,

    #[schemars(
        description = "Circulating supply (total_supply minus exchanged/deducted points)"
    )]
    pub circulating_supply: i64,
}
