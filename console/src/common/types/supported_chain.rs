use serde::{Deserialize, Serialize};

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
    Local,

    KaiaKairos,

    Kaia,
}

impl SupportedChain {
    const ALL: [Self; 3] = [Self::Local, Self::KaiaKairos, Self::Kaia];

    fn metadata(&self) -> ChainMetadata {
        match self {
            Self::Local => ChainMetadata {
                chain_id: 31337,
                name: "Local",
                explorer_url: "",
                symbol: "ETH",
                is_testnet: true,
            },
            Self::KaiaKairos => ChainMetadata {
                chain_id: 1001,
                name: "Kaia Kairos",
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

    /// Chains visible to the UI based on the current build environment.
    /// `Local` only shows up when the binary was built with `ENV=local`.
    pub fn visible() -> impl Iterator<Item = Self> {
        use crate::common::config::Environment;
        let env = Environment::default();
        Self::ALL
            .into_iter()
            .filter(move |c| !matches!(c, Self::Local) || env == Environment::Local)
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
}

pub fn get_supported_chain(chain_id: u64) -> Option<SupportedChain> {
    SupportedChain::from_chain_id(chain_id)
}

pub fn chain_display_name(chain_id: u64) -> String {
    SupportedChain::from_chain_id(chain_id)
        .map(|c| c.display_name())
        .unwrap_or_else(|| format!("Unknown ({chain_id})"))
}
