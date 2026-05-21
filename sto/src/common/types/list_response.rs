use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Default)]
#[serde(bound(deserialize = "T: serde::de::DeserializeOwned"))]
pub struct ListResponse<T>
where
    T: Clone + Serialize + serde::de::DeserializeOwned,
{
    pub items: Vec<T>,
    pub bookmark: Option<String>,
}

impl<T> PartialEq for ListResponse<T>
where
    T: Clone + PartialEq + Serialize + serde::de::DeserializeOwned,
{
    fn eq(&self, other: &Self) -> bool {
        self.items == other.items && self.bookmark == other.bookmark
    }
}

impl<T> From<(Vec<T>, Option<String>)> for ListResponse<T>
where
    T: Clone + serde::de::DeserializeOwned + Serialize,
{
    fn from((items, bookmark): (Vec<T>, Option<String>)) -> Self {
        Self { items, bookmark }
    }
}
