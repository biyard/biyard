use crate::common::*;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde_repr::Serialize_repr,
    serde_repr::Deserialize_repr,
    Default,
    DynamoEnum,
)]
#[repr(u8)]
pub enum OrganizationRole {
    #[default]
    Viewer = 1,
    Admin = 2,
    Owner = 3,
}

impl OrganizationRole {
    pub fn allows(self, required: Self) -> bool {
        (self as u8) >= (required as u8)
    }
}

