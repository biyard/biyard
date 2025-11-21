#[derive(serde::Serialize)]
pub struct InitialQuery {
    key: serde_json::Value,
    data: serde_json::Value,
}

impl InitialQuery {
    pub fn new<E>(
        key: impl serde::Serialize,
        value: impl serde::Serialize,
    ) -> Result<Self, E>
    where
        E: From<serde_json::Error>,
    {
        Ok(Self {
            key: serde_json::to_value(key)?,
            data: serde_json::to_value(value)?,
        })
    }

    pub fn new_infinite_list<E>(
        key: impl serde::Serialize,
        value: impl serde::Serialize,
        bookmark: Option<String>,
    ) -> Result<Self, E>
    where
        E: From<serde_json::Error>,
    {
        let page = serde_json::to_value(value)?;
        Ok(Self {
            key: serde_json::to_value(key)?,
            data: serde_json::json!({
                "pages": [page],
                "pageParams": [bookmark]
            }),
        })
    }
}

#[derive(serde::Serialize)]
pub struct BootData {
    react_query: Vec<InitialQuery>,
}

impl BootData {
    pub fn new(react_query: Vec<InitialQuery>) -> Self {
        Self { react_query }
    }

    pub fn to_json<E>(&self) -> Result<String, E>
    where
        E: From<serde_json::Error>,
    {
        Ok(serde_json::to_string(self)?.replace("</", "\\u003c/"))
    }
}
