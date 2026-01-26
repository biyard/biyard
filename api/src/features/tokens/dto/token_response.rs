use crate::*;

#[derive(Debug, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct TokenResponse {
    #[schemars(description = "Project ID")]
    pub pk: Partition,

    #[schemars(description = "Token name")]
    pub name: String,

    #[schemars(description = "Token symbol")]
    pub symbol: String,

    #[schemars(description = "Number of decimals")]
    pub decimals: u8,

    #[schemars(description = "Total supply")]
    pub total_supply: i64,

    #[schemars(description = "Circulating supply")]
    pub circulating_supply: i64,

    #[schemars(description = "Token description")]
    pub description: Option<String>,

    #[schemars(description = "Creation timestamp")]
    pub created_at: i64,

    #[schemars(description = "Last update timestamp")]
    pub updated_at: i64,
}

impl From<crate::features::tokens::ProjectToken> for TokenResponse {
    fn from(token: crate::features::tokens::ProjectToken) -> Self {
        Self {
            pk: token.pk,
            name: token.name,
            symbol: token.symbol,
            decimals: token.decimals,
            total_supply: token.total_supply,
            circulating_supply: token.circulating_supply,
            description: token.description,
            created_at: token.created_at,
            updated_at: token.updated_at,
        }
    }
}
