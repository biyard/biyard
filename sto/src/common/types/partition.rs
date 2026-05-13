use crate::common::*;

#[derive(
    Debug, Clone, SerializeDisplay, DeserializeFromStr, Default, DynamoEnum, PartialEq, Eq,
)]
pub enum Partition {
    #[default]
    None,

    // STO 자산 (1건당 UUID v7)
    Sto(String),

    // 발행사 (slug)
    Issuer(String),

    // 집계 row 의 partition. 단일 값 "AGGREGATE".
    Aggregate,
}

/// Sto 외부 인터페이스용 newtype (REST 경로 파라미터 등)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoPartition(pub String);

impl std::fmt::Display for StoPartition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for StoPartition {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let s = s.trim_start_matches("STO#").to_string();
        Ok(StoPartition(s))
    }
}

impl From<StoPartition> for Partition {
    fn from(p: StoPartition) -> Self {
        Partition::Sto(p.0)
    }
}

impl From<Partition> for StoPartition {
    fn from(p: Partition) -> Self {
        match p {
            Partition::Sto(id) => Self(id),
            _ => Self(String::new()),
        }
    }
}

impl From<String> for StoPartition {
    fn from(s: String) -> Self {
        let s = s.trim_start_matches("STO#").to_string();
        StoPartition(s)
    }
}
