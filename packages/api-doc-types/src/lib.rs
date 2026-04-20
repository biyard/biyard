/// Metadata for a single API endpoint, registered by the `#[api_doc]` macro.
pub struct ApiEndpointMeta {
    pub method: &'static str,
    pub path: &'static str,
    pub group: &'static str,
    pub summary: &'static str,
    pub description: &'static str,
    pub summary_ko: &'static str,
    pub description_ko: &'static str,
    pub auth: &'static str,
    pub path_params: &'static [(&'static str, &'static str)],
    pub query_params: &'static [(&'static str, &'static str, bool)],
    pub body_params: &'static [(&'static str, &'static str)],
    pub response_type: &'static str,
}

inventory::collect!(ApiEndpointMeta);

/// i18n field docs: (field_name, en_description, ko_description)
pub type FieldDocs = &'static [(&'static str, &'static str, &'static str)];

/// Schema entry for a DTO type, registered by the `ApiDocSchema` derive macro.
pub struct ApiSchemaEntry {
    pub type_name: &'static str,
    pub field_docs: fn() -> FieldDocs,
    pub schema_fn: Option<fn() -> serde_json::Value>,
}

inventory::collect!(ApiSchemaEntry);

/// Look up i18n field docs for a given type name.
pub fn field_docs_for_type(type_name: &str) -> Option<FieldDocs> {
    let inner = strip_wrappers(type_name);
    inventory::iter::<ApiSchemaEntry>
        .into_iter()
        .find(|e| e.type_name == inner)
        .map(|e| (e.field_docs)())
}

/// Look up JSON Schema for a given type name (server-only, via schemars).
pub fn schema_for_type(type_name: &str) -> Option<serde_json::Value> {
    let inner = strip_wrappers(type_name);
    inventory::iter::<ApiSchemaEntry>
        .into_iter()
        .find(|e| e.type_name == inner)
        .and_then(|e| e.schema_fn.map(|f| f()))
}

/// Look up JSON Schema + i18n docs merged for a given type name.
pub fn schema_with_i18n(type_name: &str) -> Option<(serde_json::Value, FieldDocs)> {
    let inner = strip_wrappers(type_name);
    let entry = inventory::iter::<ApiSchemaEntry>
        .into_iter()
        .find(|e| e.type_name == inner)?;
    let schema = (entry.schema_fn?)(  );
    let docs = (entry.field_docs)();
    Some((schema, docs))
}

fn strip_wrappers(type_name: &str) -> &str {
    type_name
        .strip_prefix("Vec<")
        .and_then(|s| s.strip_suffix('>'))
        .or_else(|| {
            type_name
                .strip_prefix("Option<")
                .and_then(|s| s.strip_suffix('>'))
        })
        .unwrap_or(type_name)
}

/// Collect all registered API endpoint metadata.
pub fn all_api_docs() -> Vec<&'static ApiEndpointMeta> {
    inventory::iter::<ApiEndpointMeta>.into_iter().collect()
}
