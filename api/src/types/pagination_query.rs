use crate::*;

pub type PaginationQuery = Query<Pagination>;

#[derive(
    serde::Deserialize,
    serde::Serialize,
    Debug,
    Clone,
    Default,
    schemars::JsonSchema,
    aide::OperationIo,
    Validate,
)]
pub struct Pagination {
    #[schemars(description = "Bookmark to start from")]
    pub bookmark: Option<String>,

    #[validate(range(min = 1, max = 100))]
    #[schemars(description = "Maximum number of items to return")]
    #[serde(
        default = "default_limit",
        deserialize_with = "deserialize_string_or_i32"
    )]
    pub limit: i32,
}

pub fn default_limit() -> i32 {
    10
}

fn deserialize_string_or_i32<'de, D>(deserializer: D) -> std::result::Result<i32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{self, Visitor};

    struct StringOrI32Visitor;

    impl<'de> Visitor<'de> for StringOrI32Visitor {
        type Value = i32;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string or integer")
        }

        fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
        where
            E: de::Error,
        {
            i32::try_from(v).map_err(|_| E::custom(format!("i32 out of range: {}", v)))
        }

        fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
        where
            E: de::Error,
        {
            i32::try_from(v).map_err(|_| E::custom(format!("i32 out of range: {}", v)))
        }

        fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
        where
            E: de::Error,
        {
            v.parse::<i32>()
                .map_err(|_| E::custom(format!("invalid integer: {}", v)))
        }
    }

    deserializer.deserialize_any(StringOrI32Visitor)
}
