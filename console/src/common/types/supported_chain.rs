use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StableTokenOption {
    pub name: &'static str,
    pub symbol: &'static str,
    pub address: &'static str,
    /// Whether the stable token has a public faucet `mint()` function
    /// (e.g. BUSDT on testnets). When `true`, the console can mint
    /// arbitrary amounts for demo/testing purposes.
    pub mintable: bool,
}

#[derive(Debug, Clone, Copy)]
struct ChainMetadata {
    chain_id: u64,
    name: &'static str,
    explorer_url: &'static str,
    symbol: &'static str,
    is_testnet: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SupportedChain {
    KaiaKairos,

    Kaia,
}

impl SupportedChain {
    const ALL: [Self; 2] = [Self::KaiaKairos, Self::Kaia];

    fn metadata(&self) -> ChainMetadata {
        match self {
            Self::KaiaKairos => ChainMetadata {
                chain_id: 1001,
                name: "Kairos",
                explorer_url: "https://kairos.kaiascan.io",
                symbol: "KAIA",
                is_testnet: true,
            },
            Self::Kaia => ChainMetadata {
                chain_id: 8217,
                name: "Kaia",
                explorer_url: "https://kaiascan.io",
                symbol: "KAIA",
                is_testnet: false,
            },
        }
    }

    pub fn chain_id(&self) -> u64 {
        self.metadata().chain_id
    }

    pub fn name(&self) -> &'static str {
        self.metadata().name
    }

    pub fn explorer_url(&self) -> &'static str {
        self.metadata().explorer_url
    }

    pub fn symbol(&self) -> &'static str {
        self.metadata().symbol
    }

    pub fn is_testnet(&self) -> bool {
        self.metadata().is_testnet
    }

    pub fn from_chain_id(chain_id: u64) -> Option<Self> {
        Self::ALL.into_iter().find(|c| c.chain_id() == chain_id)
    }

    pub fn all() -> impl Iterator<Item = Self> {
        Self::ALL.into_iter()
    }

    pub fn visible() -> impl Iterator<Item = Self> {
        Self::ALL.into_iter()
    }

    pub fn testnets() -> impl Iterator<Item = Self> {
        Self::ALL.into_iter().filter(|c| c.is_testnet())
    }

    pub fn mainnets() -> impl Iterator<Item = Self> {
        Self::ALL.into_iter().filter(|c| !c.is_testnet())
    }

    pub fn display_name(&self) -> String {
        if self.is_testnet() {
            format!("{} (Testnet, {})", self.name(), self.chain_id())
        } else {
            format!("{} ({})", self.name(), self.chain_id())
        }
    }

    pub fn explorer_tx_url(&self, tx_hash: &str) -> String {
        format!("{}/tx/{}", self.explorer_url(), tx_hash)
    }

    pub fn explorer_address_url(&self, address: &str) -> String {
        format!("{}/address/{}", self.explorer_url(), address)
    }

    pub fn stable_token_options(&self) -> Vec<StableTokenOption> {
        match self {
            Self::KaiaKairos => vec![
                StableTokenOption {
                    name: "BUSDT (Test Faucet)",
                    symbol: "BUSDT",
                    address: "0xa3B7946A9B58B0b2547086f9677c6964739bf5Cd",
                    mintable: true,
                },
                StableTokenOption {
                    name: "USDT",
                    symbol: "USDT",
                    address: "0xd077a400968890eacc75cdc901f0356c943e4fdb",
                    mintable: false,
                },
            ],
            Self::Kaia => vec![StableTokenOption {
                name: "USDT",
                symbol: "USDT",
                address: "0xd077a400968890eacc75cdc901f0356c943e4fdb",
                mintable: false,
            }],
        }
    }
}

pub fn get_supported_chain(chain_id: u64) -> Option<SupportedChain> {
    SupportedChain::from_chain_id(chain_id)
}

pub fn chain_display_name(chain_id: u64) -> String {
    SupportedChain::from_chain_id(chain_id)
        .map(|c| c.display_name())
        .unwrap_or_else(|| format!("Unknown ({chain_id})"))
}
