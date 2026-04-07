use crate::common::*;

#[derive(
    Debug, Clone, SerializeDisplay, DeserializeFromStr, Default, DynamoEnum, PartialEq, Eq,
)]
pub enum Partition {
    #[default]
    None,

    // Account
    Account(String),

    // Session
    Session(String),

    // Credential
    Credential(String),

    // Enterprise
    Enterprise(String),

    // Project
    Project(String),
    MonthlyPoints(String),

    // Point Feature
    MetaUser(String),

    // Token
    Token(String),

    // Token Balance
    TokenBalance(String),

    // Contact
    Contact(String),

    // Update
    Update(String),
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectPartition(pub String);

impl std::fmt::Display for ProjectPartition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for ProjectPartition {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let s = if s.starts_with("PROJECT#") {
            s.replace("PROJECT#", "")
        } else {
            s.to_string()
        };
        Ok(ProjectPartition(s))
    }
}

impl Into<Partition> for ProjectPartition {
    fn into(self) -> Partition {
        Partition::Project(self.0)
    }
}

impl From<Partition> for ProjectPartition {
    fn from(partition: Partition) -> Self {
        match partition {
            Partition::Project(id) => Self(id),
            _ => Self(String::new()),
        }
    }
}

impl From<String> for ProjectPartition {
    fn from(s: String) -> Self {
        let s = if s.starts_with("PROJECT#") {
            s.replace("PROJECT#", "")
        } else {
            s
        };
        ProjectPartition(s)
    }
}
