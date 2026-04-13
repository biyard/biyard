use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DistributionSlotEntry {
    pub wallet: String,
    pub bps: u16,
}
