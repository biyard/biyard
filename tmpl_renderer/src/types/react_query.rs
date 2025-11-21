#[derive(serde::Serialize)]
pub struct InitialQuery {
    key: serde_json::Value,
    data: serde_json::Value,
}

#[derive(serde::Serialize)]
pub struct BootData {
    react_query: Vec<InitialQuery>,
}

impl BootData {
    pub fn new() -> Self {
        Self {
            react_query: Vec::new(),
        }
    }

    pub fn add_query(
        &mut self,
        key: impl serde::Serialize,
        value: impl serde::Serialize,
    ) -> Result<(), serde_json::Error> {
        let query = InitialQuery {
            key: serde_json::to_value(key)?,
            data: serde_json::to_value(value)?,
        };
        self.react_query.push(query);
        Ok(())
    }

    pub fn add_infinite_list_query(
        &mut self,
        key: impl serde::Serialize,
        value: impl serde::Serialize,
        bookmark: Option<String>,
    ) -> Result<(), serde_json::Error> {
        let page = serde_json::to_value(value)?;
        let query = InitialQuery {
            key: serde_json::to_value(key)?,
            data: serde_json::json!({
                "pages": [page],
                "pageParams": [bookmark]
            }),
        };
        self.react_query.push(query);
        Ok(())
    }

    pub fn to_json<E>(self) -> Result<String, E>
    where
        E: From<serde_json::Error>,
    {
        Ok(serde_json::to_string(&self)?.replace("</", "\\u003c/"))
    }
}
