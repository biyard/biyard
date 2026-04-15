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

/// Field metadata tuple: (name, type_str, required, en_description, ko_description)
pub type FieldMeta = &'static [(&'static str, &'static str, bool, &'static str, &'static str)];

/// Schema entry for a DTO type, registered by the `ApiDocSchema` derive macro.
pub struct ApiSchemaEntry {
    pub type_name: &'static str,
    pub field_meta: fn() -> FieldMeta,
}

inventory::collect!(ApiSchemaEntry);

/// Look up field metadata for a given type name.
///
/// Strips wrapper types like `Vec<Foo>` and `Option<Foo>` to find the
/// inner type name registered by `#[derive(ApiDocSchema)]`.
pub fn field_meta_for_type(type_name: &str) -> Option<FieldMeta> {
    let inner = type_name
        .strip_prefix("Vec<")
        .and_then(|s| s.strip_suffix('>'))
        .or_else(|| {
            type_name
                .strip_prefix("Option<")
                .and_then(|s| s.strip_suffix('>'))
        })
        .unwrap_or(type_name);

    inventory::iter::<ApiSchemaEntry>
        .into_iter()
        .find(|e| e.type_name == inner)
        .map(|e| (e.field_meta)())
}

/// Collect all registered API endpoint metadata.
pub fn all_api_docs() -> Vec<&'static ApiEndpointMeta> {
    inventory::iter::<ApiEndpointMeta>.into_iter().collect()
}
