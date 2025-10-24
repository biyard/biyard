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
    pub limit: Option<i32>,
}
