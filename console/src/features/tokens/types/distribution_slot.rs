use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub struct DistributionSlotEntry {
    /// Wallet address to receive tokens.
    pub wallet: String,
    /// Share in basis points (e.g. 5000 = 50%).
    pub bps: u16,
}
