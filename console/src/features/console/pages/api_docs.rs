use dioxus::prelude::*;
use dioxus_translate::use_translate;
use serde::{Deserialize, Serialize};

use crate::features::console::i18n::ApiDocsTranslate;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SchemaField {
    pub name: String,
    pub field_type: String,
    pub required: bool,
    pub description: String,
    pub description_ko: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sub_fields: Vec<SchemaField>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub enum_variants: Vec<EnumVariantInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnumVariantInfo {
    pub name: String,
    pub fields: Vec<SchemaField>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EndpointInfo {
    pub method: String,
    pub path: String,
    pub group: String,
    pub summary: String,
    pub summary_ko: String,
    pub description: String,
    pub description_ko: String,
    pub auth: String,
    pub path_params: Vec<(String, String)>,
    pub query_params: Vec<(String, String, bool)>,
    pub body_params: Vec<(String, String)>,
    pub response_type: String,
    pub body_schema: Vec<SchemaField>,
    pub response_schema: Vec<SchemaField>,
}

#[server]
async fn get_api_endpoints() -> Result<Vec<EndpointInfo>, ServerFnError> {
    use crate::common::types::api_doc_meta;

    let mut endpoints: Vec<EndpointInfo> = api_doc_meta::all_api_docs()
        .into_iter()
        .map(|ep| {
            let body_type = ep.body_params.first().map(|(_, t)| *t).unwrap_or_default();
            let body_schema = extract_schema_fields(body_type);
            let response_schema = extract_schema_fields(ep.response_type);

            EndpointInfo {
                method: ep.method.to_string(),
                path: ep.path.to_string(),
                group: ep.group.to_string(),
                summary: ep.summary.to_string(),
                summary_ko: ep.summary_ko.to_string(),
                description: ep.description.to_string(),
                description_ko: ep.description_ko.to_string(),
                auth: ep.auth.to_string(),
                path_params: ep
                    .path_params
                    .iter()
                    .map(|(n, t)| (n.to_string(), t.to_string()))
                    .collect(),
                query_params: ep
                    .query_params
                    .iter()
                    .map(|(n, t, o)| (n.to_string(), t.to_string(), *o))
                    .collect(),
                body_params: ep
                    .body_params
                    .iter()
                    .map(|(n, t)| (n.to_string(), t.to_string()))
                    .collect(),
                response_type: ep.response_type.to_string(),
                body_schema,
                response_schema,
            }
        })
        .collect();

    endpoints.sort_by(|a, b| {
        a.group
            .cmp(&b.group)
            .then(method_order(&a.method).cmp(&method_order(&b.method)))
            .then(a.path.cmp(&b.path))
    });
    Ok(endpoints)
}

#[cfg(feature = "server")]
fn extract_schema_fields(type_name: &str) -> Vec<SchemaField> {
    use crate::common::types::api_doc_meta;

    let (schema_json, field_docs) = match api_doc_meta::schema_with_i18n(type_name) {
        Some(v) => v,
        None => return vec![],
    };

    let mut fields = vec![];

    let required_fields: Vec<String> = schema_json
        .get("required")
        .and_then(|r| r.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
        .unwrap_or_default();

    if let Some(properties) = schema_json.get("properties").and_then(|p| p.as_object()) {
        for (name, prop) in properties {
            let field_type = json_schema_type_to_string(prop);
            let (en_desc, ko_desc) = field_docs
                .iter()
                .find(|(n, _, _)| *n == name)
                .map(|(_, en, ko)| (en.to_string(), ko.to_string()))
                .unwrap_or_default();

            let sub_fields = extract_sub_fields(prop);
            let enum_variants = extract_enum_variants(prop);

            fields.push(SchemaField {
                name: name.clone(),
                field_type,
                required: required_fields.contains(name),
                description: en_desc,
                description_ko: ko_desc,
                sub_fields,
                enum_variants,
            });
        }
    }

    // Handle oneOf (e.g. flatten + tag enums)
    if schema_json.get("oneOf").is_some() {
        if let Some(one_of) = schema_json.get("oneOf").and_then(|o| o.as_array()) {
            // Pass common fields so they get merged into each variant
            let common_fields: Vec<SchemaField> = fields.clone();
            let variants = extract_one_of_variants(one_of, field_docs, &common_fields);
            if !variants.is_empty() {
                // Remove common fields from top level — they're now inside each variant
                fields.clear();
                fields.push(SchemaField {
                    name: "__variants__".to_string(),
                    field_type: String::new(),
                    required: false,
                    description: String::new(),
                    description_ko: String::new(),
                    sub_fields: vec![],
                    enum_variants: variants,
                });
            }
        }
    }

    fields.sort_by(|a, b| b.required.cmp(&a.required).then(a.name.cmp(&b.name)));
    fields
}

#[cfg(feature = "server")]
fn extract_sub_fields(prop: &serde_json::Value) -> Vec<SchemaField> {
    // Check for array items with properties
    if let Some(items) = prop.get("items") {
        if let Some(props) = items.get("properties").and_then(|p| p.as_object()) {
            let req: Vec<String> = items
                .get("required")
                .and_then(|r| r.as_array())
                .map(|a| a.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default();
            return props
                .iter()
                .map(|(n, p)| SchemaField {
                    name: n.clone(),
                    field_type: json_schema_type_to_string(p),
                    required: req.contains(n),
                    description: p.get("description").and_then(|d| d.as_str()).unwrap_or("").to_string(),
                    description_ko: String::new(),
                    sub_fields: vec![],
                    enum_variants: vec![],
                })
                .collect();
        }
    }
    vec![]
}

#[cfg(feature = "server")]
fn extract_enum_variants(prop: &serde_json::Value) -> Vec<EnumVariantInfo> {
    // Check for oneOf in the property itself (e.g. enum field)
    if let Some(one_of) = prop.get("oneOf").and_then(|o| o.as_array()) {
        // Simple string enum: [{"const": "Award"}, {"const": "Deduct"}]
        let consts: Vec<String> = one_of
            .iter()
            .filter_map(|v| v.get("const").and_then(|c| c.as_str()).map(String::from))
            .collect();
        if !consts.is_empty() {
            return consts
                .into_iter()
                .map(|name| EnumVariantInfo { name, fields: vec![] })
                .collect();
        }
    }

    // Check for enum array
    if let Some(enum_vals) = prop.get("enum").and_then(|e| e.as_array()) {
        return enum_vals
            .iter()
            .filter_map(|v| v.as_str().map(|s| EnumVariantInfo { name: s.to_string(), fields: vec![] }))
            .collect();
    }

    vec![]
}

#[cfg(feature = "server")]
fn extract_one_of_variants(
    one_of: &[serde_json::Value],
    field_docs: api_doc_types::FieldDocs,
    common_fields: &[SchemaField],
) -> Vec<EnumVariantInfo> {
    let mut variants = vec![];

    for variant_schema in one_of {
        let properties = match variant_schema.get("properties").and_then(|p| p.as_object()) {
            Some(p) => p,
            None => continue,
        };

        let req: Vec<String> = variant_schema
            .get("required")
            .and_then(|r| r.as_array())
            .map(|a| a.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();

        // Find the discriminator tag field name and variant value
        let (tag_name, variant_name) = properties
            .iter()
            .find_map(|(name, v)| {
                v.get("const")
                    .and_then(|c| c.as_str())
                    .map(|val| (name.clone(), val.to_string()))
            })
            .unwrap_or_default();

        // Build fields: tag field first, then variant-specific fields, then common fields
        let mut fields: Vec<SchemaField> = vec![];

        if !tag_name.is_empty() {
            fields.push(SchemaField {
                name: tag_name.clone(),
                field_type: format!("\"{}\"", variant_name),
                required: true,
                description: String::new(),
                description_ko: String::new(),
                sub_fields: vec![],
                enum_variants: vec![],
            });
        }

        for (n, p) in properties {
            if *n == tag_name {
                continue;
            }
            let (en_desc, ko_desc) = field_docs
                .iter()
                .find(|(dn, _, _)| *dn == n)
                .map(|(_, en, ko)| (en.to_string(), ko.to_string()))
                .unwrap_or_default();
            fields.push(SchemaField {
                name: n.clone(),
                field_type: json_schema_type_to_string(p),
                required: req.contains(n),
                description: en_desc,
                description_ko: ko_desc,
                sub_fields: vec![],
                enum_variants: vec![],
            });
        }

        // Append common fields (from the parent struct, e.g. month, description)
        for cf in common_fields {
            if !fields.iter().any(|f| f.name == cf.name) {
                fields.push(cf.clone());
            }
        }

        if !variant_name.is_empty() {
            variants.push(EnumVariantInfo { name: variant_name, fields });
        }
    }

    variants
}

#[cfg(feature = "server")]
fn json_schema_type_to_string(prop: &serde_json::Value) -> String {
    // Handle "type": "string" (single string)
    if let Some(ty) = prop.get("type").and_then(|t| t.as_str()) {
        return match ty {
            "string" => "string".to_string(),
            "integer" => "integer".to_string(),
            "number" => "number".to_string(),
            "boolean" => "boolean".to_string(),
            "array" => {
                if let Some(items) = prop.get("items") {
                    let inner = json_schema_type_to_string(items);
                    format!("{}[]", inner)
                } else {
                    "array".to_string()
                }
            }
            "object" => "object".to_string(),
            _ => ty.to_string(),
        };
    }

    // Handle "type": ["string", "null"] (schemars Option<T> output)
    if let Some(types) = prop.get("type").and_then(|t| t.as_array()) {
        let non_null: Vec<&str> = types
            .iter()
            .filter_map(|v| v.as_str())
            .filter(|s| *s != "null")
            .collect();
        if let Some(ty) = non_null.first() {
            return ty.to_string();
        }
    }

    if prop.get("const").is_some() {
        "string".to_string()
    } else if prop.get("oneOf").is_some() || prop.get("anyOf").is_some() {
        "enum".to_string()
    } else if prop.get("enum").is_some() {
        "enum".to_string()
    } else if let Some(r) = prop.get("$ref").and_then(|r| r.as_str()) {
        r.rsplit('/').next().unwrap_or("object").to_string()
    } else {
        "any".to_string()
    }
}

fn is_ko(lang: &dioxus_translate::Language) -> bool {
    matches!(lang, dioxus_translate::Language::Ko)
}

// ── Page ───────────────────────────────────────────────────────────

#[component]
pub fn ApiDocs() -> Element {
    let t: ApiDocsTranslate = use_translate();
    let lang = dioxus_translate::use_language();
    let mut sidebar_width = use_signal(|| 224i32); // 14rem = 224px (w-56)
    let mut dragging = use_signal(|| false);

    let on_pointer_down = move |e: PointerEvent| {
        e.prevent_default();
        dragging.set(true);
    };
    let on_pointer_move = move |e: PointerEvent| {
        if dragging() {
            let x = e.page_coordinates().x as i32;
            sidebar_width.set(x.clamp(160, 480));
        }
    };
    let on_pointer_up = move |_: PointerEvent| {
        dragging.set(false);
    };

    let endpoints = use_server_future(get_api_endpoints)?;

    let endpoints = match endpoints() {
        Some(Ok(eps)) => eps,
        Some(Err(_)) => {
            return rsx! {
                div { class: "p-8 text-foreground", "Failed to load." }
            };
        }
        None => {
            return rsx! {
                div { class: "p-8 text-foreground-muted", "Loading..." }
            };
        }
    };

    let ko = is_ko(&lang());

    let mut groups: Vec<(String, Vec<&EndpointInfo>)> = Vec::new();
    for ep in &endpoints {
        if let Some(g) = groups.iter_mut().find(|(name, _)| name == &ep.group) {
            g.1.push(ep);
        } else {
            groups.push((ep.group.clone(), vec![ep]));
        }
    }

    rsx! {
        div { class: "min-h-screen bg-background text-foreground",
            // ── Sticky header ──
            header { class: "border-b border-border bg-background/95 backdrop-blur sticky top-0 z-20",
                div { class: "w-full px-6 h-14 flex items-center justify-between",
                    div { class: "flex items-center gap-3",
                        span { class: "font-semibold text-lg tracking-tight", "Biyard" }
                        span { class: "text-foreground-muted", "/" }
                        span { class: "text-foreground-muted text-sm", {t.api_docs_title} }
                    }
                    a {
                        href: "/docs/api.json",
                        target: "_blank",
                        class: "text-sm text-brand hover:text-brand-strong",
                        "OpenAPI JSON"
                    }
                }
            }

            // ── Two-column shell ──
            div {
                class: if dragging() { "w-full flex select-none" } else { "w-full flex" },
                onpointermove: on_pointer_move,
                onpointerup: on_pointer_up,

                // Left sidebar nav
                nav {
                    class: "hidden lg:flex lg:flex-col shrink-0 sticky top-14 h-[calc(100vh-3.5rem)] relative",
                    style: "width: {sidebar_width()}px",
                    div { class: "flex-1 overflow-y-auto py-6 px-4 border-r border-border",
                        for (group_name , group_eps) in &groups {
                            div { class: "mb-5",
                                h3 { class: "text-xs font-semibold text-foreground-muted uppercase tracking-wider mb-2 px-2",
                                    {group_name.clone()}
                                }
                                for ep in group_eps {
                                    a {
                                        href: "#ep-{ep.method}-{ep.path}",
                                        class: "block px-2 py-1.5 text-sm text-foreground-soft hover:text-foreground hover:bg-background-muted rounded truncate",
                                        title: "{ep.path}",
                                        span { class: "inline-block w-[6ch] text-center font-mono text-[10px] font-semibold rounded px-1 py-0.5 mr-1.5 {method_badge_class(&ep.method)}",
                                            {ep.method.clone()}
                                        }
                                        {
                                            if ko && !ep.summary_ko.is_empty() {
                                                ep.summary_ko.clone()
                                            } else {
                                                ep.summary.clone()
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    // Language switcher
                    div { class: "border-t border-r border-border px-4 py-3", LanguageSwitcher {} }
                    // Resize handle
                    div {
                        class: "absolute top-0 right-0 w-1 h-full cursor-col-resize hover:bg-brand/30 active:bg-brand/50 transition-colors",
                        onpointerdown: on_pointer_down,
                    }
                }

                // Main content
                main { class: "flex-1 min-w-0",
                    // ── Hero / Auth section ──
                    div { class: "border-b border-border py-10 px-6 lg:px-10",
                        div { class: "grid lg:grid-cols-2 gap-8",
                            div {
                                h1 { class: "text-3xl font-bold mb-4", "Biyard API" }
                                p { class: "text-foreground-muted leading-relaxed mb-6",
                                    {t.api_docs_hero_desc}
                                }
                                div { class: "flex items-center gap-2 text-sm",
                                    span { class: "text-foreground-muted", {t.api_docs_base_url_label} }
                                    code { class: "font-mono text-sm bg-panel-strong px-2 py-1 rounded",
                                        "https://api.biyard.co"
                                    }
                                }
                            }
                            div { class: "bg-[#0a0e1a] rounded-xl overflow-hidden",
                                div { class: "px-4 py-2.5 border-b border-gray-800 flex items-center gap-2",
                                    span { class: "text-xs text-gray-400 font-medium",
                                        {t.api_docs_auth_title}
                                    }
                                }
                                div { class: "px-4 py-3 text-sm text-gray-400",
                                    {t.api_docs_auth_desc}
                                }
                                pre { class: "px-4 pb-4 text-sm font-mono leading-relaxed overflow-x-auto",
                                    span { class: "text-gray-500",
                                        "curl https://api.biyard.co/v1/... \\\n"
                                    }
                                    span { class: "text-gray-500", "  -H " }
                                    span { class: "text-green-400", "\"Authorization: Bearer " }
                                    span { class: "text-yellow-300 italic", "sk_live_..." }
                                    span { class: "text-green-400", "\"" }
                                }
                            }
                        }
                    }

                    // ── Endpoints ──
                    for (group_name , group_endpoints) in &groups {
                        div { class: "border-b border-border",
                            div { class: "px-6 lg:px-10 pt-10 pb-2 flex items-baseline gap-3",
                                h2 { class: "text-xl font-bold", {group_name.clone()} }
                                span { class: "text-sm text-foreground-muted",
                                    "{group_endpoints.len()} {t.api_docs_endpoints_count}"
                                }
                            }
                            for ep in group_endpoints {
                                EndpointSection { endpoint: (*ep).clone(), ko }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(feature = "server")]
fn method_order(method: &str) -> u8 {
    match method {
        "GET" => 0,
        "POST" => 1,
        "PUT" => 2,
        "PATCH" => 3,
        "DELETE" => 4,
        _ => 5,
    }
}

fn method_badge_class(method: &str) -> &'static str {
    match method {
        "GET" => "bg-success/15 text-success",
        "POST" => "bg-brand/15 text-brand",
        "PUT" => "bg-warning/15 text-warning",
        "PATCH" => "bg-warning/15 text-warning",
        "DELETE" => "bg-danger/15 text-danger",
        _ => "bg-foreground-muted/15 text-foreground-muted",
    }
}

fn method_bg(method: &str) -> &'static str {
    match method {
        "GET" => "bg-success",
        "POST" => "bg-brand",
        "PUT" => "bg-warning",
        "PATCH" => "bg-warning",
        "DELETE" => "bg-danger",
        _ => "bg-foreground-muted",
    }
}

fn auth_to_permission(auth: &str) -> &'static str {
    match auth {
        "ProjectViewerAuth" => "Viewer",
        "ProjectAdminAuth" => "Admin",
        "ProjectAuth" => "Owner",
        _ => "Authenticated",
    }
}

fn build_curl_example(ep: &EndpointInfo) -> String {
    let api_path = ep
        .path
        .split('?')
        .next()
        .unwrap_or(&ep.path)
        .replace(":project_id", "{project_id}")
        .replace(":meta_user_id", "{meta_user_id}");

    let mut curl = if ep.method == "GET" {
        format!("curl https://api.biyard.co{}", api_path)
    } else {
        format!("curl -X {} https://api.biyard.co{}", ep.method, api_path)
    };

    curl.push_str(" \\\n  -H \"Authorization: Bearer sk_live_...\"");

    if !ep.body_params.is_empty() || !ep.body_schema.is_empty() {
        curl.push_str(" \\\n  -H \"Content-Type: application/json\"");
        curl.push_str(" \\\n  -d '");

        if !ep.body_schema.is_empty() {
            let is_array_body = ep
                .body_params
                .first()
                .map(|(_, t)| t.starts_with("Vec<"))
                .unwrap_or(false);

            if is_array_body {
                let obj = build_object_example(&ep.body_schema, 2);
                curl.push_str(&format!("[\n  {}\n]", obj));
            } else {
                let obj = build_object_example(&ep.body_schema, 1);
                curl.push_str(&obj);
            }
        } else {
            curl.push_str("{}");
        }
        curl.push('\'');
    }

    curl
}

fn build_response_example(ep: &EndpointInfo) -> String {
    if ep.response_schema.is_empty() {
        return "{}".to_string();
    }

    if ep.response_type.starts_with("Vec<") {
        let obj = build_object_example(&ep.response_schema, 2);
        format!("[\n  {}\n]", obj)
    } else {
        build_object_example(&ep.response_schema, 1)
    }
}

fn build_object_example(fields: &[SchemaField], depth: usize) -> String {
    // If the only field is __variants__, use the first variant's fields as the example
    let effective_fields: Vec<&SchemaField> =
        if fields.len() == 1 && fields[0].name.starts_with("__") && !fields[0].enum_variants.is_empty() {
            fields[0].enum_variants[0].fields.iter().collect()
        } else {
            fields.iter().filter(|f| !f.name.is_empty() && !f.name.starts_with("__")).collect()
        };

    let indent = "  ".repeat(depth);
    let outer = "  ".repeat(depth - 1);

    let entries: Vec<String> = effective_fields
        .iter()
        .map(|f| {
            let val = field_example_value(f, depth);
            format!("{}\"{}\": {}", indent, f.name, val)
        })
        .collect();

    format!("{{\n{}\n{}}}", entries.join(",\n"), outer)
}

fn field_example_value(f: &SchemaField, depth: usize) -> String {
    if !f.required && !has_meaningful_example(&f.name) {
        if f.field_type.ends_with("[]") && !f.sub_fields.is_empty() {
            // still show the shape even if optional
        } else {
            return "null".to_string();
        }
    }

    if f.field_type.ends_with("[]") && !f.sub_fields.is_empty() {
        let obj = build_object_example(&f.sub_fields, depth + 2);
        let inner_indent = "  ".repeat(depth + 1);
        let outer_indent = "  ".repeat(depth);
        return format!("[\n{}{}\n{}]", inner_indent, obj, outer_indent);
    }

    // Tag field: type is a quoted value like "\"Award\""
    if f.field_type.starts_with('"') {
        return f.field_type.clone();
    }

    match f.field_type.as_str() {
        "string" => string_example(&f.name),
        "integer" => integer_example(&f.name),
        "number" => number_example(&f.name),
        "boolean" => boolean_example(&f.name),
        _ if f.field_type.ends_with("[]") => "[]".to_string(),
        _ => "null".to_string(),
    }
}

fn has_meaningful_example(name: &str) -> bool {
    matches!(
        name,
        "month" | "date" | "description" | "memo" | "to" | "from"
            | "start_month" | "last_minted_month" | "name" | "symbol"
    )
}

fn string_example(name: &str) -> String {
    match name {
        "id" | "pk" | "project_id" => "\"019073a1-...8a0f\"".to_string(),
        "account_id" => "\"01907c2d-...8b7d\"".to_string(),
        "organization_id" => "\"019084f3-...6d9e\"".to_string(),
        "transaction_id" => "\"01909e5a-...5a2e\"".to_string(),
        "name" => "\"My Brand Token\"".to_string(),
        "symbol" => "\"MBT\"".to_string(),
        "description" => "\"A loyalty points program.\"".to_string(),
        "email" => "\"admin@example.com\"".to_string(),
        "meta_user_id" => "\"usr_01907f8a-...7a3f\"".to_string(),
        "month" | "date" => "\"2025-04\"".to_string(),
        "start_month" | "last_minted_month" => "\"2025-01\"".to_string(),
        "status" => "\"Active\"".to_string(),
        "transaction_type" => "\"Award\"".to_string(),
        "wallet" => "\"0x742d...bD18\"".to_string(),
        "to" => "\"usr_01908b4d-...4d6b\"".to_string(),
        "from" => "\"usr_01907f8a-...7a3f\"".to_string(),
        "memo" => "\"Weekly engagement reward\"".to_string(),
        "target_user_id" => "\"usr_01908b4d-...4d6b\"".to_string(),
        n if n.contains("address") => "\"0xA0b8...eB48\"".to_string(),
        n if n.contains("tx_hash") => "\"0x8bad...7c8d\"".to_string(),
        n if n.contains("url") => "\"https://cdn.example.com/logo.png\"".to_string(),
        n if n.contains("symbol") => "\"BUSDT\"".to_string(),
        n if n.ends_with("_raw") || n.ends_with("_1e18") => "\"1000000...000\"".to_string(),
        _ => "\"...\"".to_string(),
    }
}

fn integer_example(name: &str) -> String {
    match name {
        "created_at" => "1713168000".to_string(),
        "updated_at" => "1713254400".to_string(),
        "expired_at" => "1713340800".to_string(),
        "balance" => "4850".to_string(),
        "total_earned" => "5200".to_string(),
        "total_spent" => "350".to_string(),
        "amount" => "150".to_string(),
        "circulating_supply" => "2500000".to_string(),
        "monthly_emission" => "100000".to_string(),
        "monthly_token_supply" => "50000".to_string(),
        "project_total_points" => "128000".to_string(),
        "chain_id" => "1001".to_string(),
        "treasury_reserve_bps" => "2000".to_string(),
        "decay_rate_bps" => "500".to_string(),
        "bps" => "5000".to_string(),
        "current_month" => "4".to_string(),
        "supplied_points" => "75000".to_string(),
        "traded_points" => "12000".to_string(),
        "awarded_points" => "50000".to_string(),
        "deducted_points" => "8000".to_string(),
        "exchanged_points" => "5000".to_string(),
        n if n.contains("decimals") => "18".to_string(),
        _ => "0".to_string(),
    }
}

fn number_example(name: &str) -> String {
    match name {
        "treasury_reserve_rate" => "0.2".to_string(),
        _ => "0.0".to_string(),
    }
}

fn boolean_example(name: &str) -> String {
    match name {
        "deployed" | "stable_mintable" => "true".to_string(),
        "exchanged" | "deploying" => "false".to_string(),
        _ => "false".to_string(),
    }
}

// ── Stripe-style endpoint section (two-column) ────────────────────

#[component]
fn EndpointSection(endpoint: EndpointInfo, ko: bool) -> Element {
    let t: ApiDocsTranslate = use_translate();
    let curl = build_curl_example(&endpoint);
    let response_json = build_response_example(&endpoint);
    let permission = auth_to_permission(&endpoint.auth);
    let method_dot = method_bg(&endpoint.method);

    let display_summary = if ko && !endpoint.summary_ko.is_empty() {
        endpoint.summary_ko.clone()
    } else {
        endpoint.summary.clone()
    };
    let display_desc = if ko && !endpoint.description_ko.is_empty() {
        endpoint.description_ko.clone()
    } else {
        endpoint.description.clone()
    };

    rsx! {
        div {
            id: "ep-{endpoint.method}-{endpoint.path}",
            class: "scroll-mt-14 grid lg:grid-cols-2 border-t border-border",

            // ── LEFT: description + parameters ──
            div { class: "px-6 lg:px-10 py-8 lg:border-r border-border",
                div { class: "flex items-center gap-2.5 mb-1",
                    span { class: "w-2 h-2 rounded-full {method_dot}" }
                    h3 { class: "font-semibold text-base", {display_summary} }
                }
                div { class: "flex items-center gap-2 mb-4",
                    code { class: "text-xs font-mono text-foreground-muted",
                        "{endpoint.method} {endpoint.path}"
                    }
                    span { class: "text-xs text-foreground-muted border border-border rounded px-1.5 py-0.5",
                        {permission}
                    }
                }

                if !display_desc.is_empty() {
                    p { class: "text-sm text-foreground-muted leading-relaxed mb-6",
                        {display_desc}
                    }
                }

                if !endpoint.path_params.is_empty() {
                    div { class: "mt-4",
                        h4 { class: "text-xs font-semibold text-foreground-muted uppercase tracking-wider mb-3",
                            {t.api_docs_path_params}
                        }
                        div { class: "divide-y divide-border border-t border-border",
                            for (name , _ty) in &endpoint.path_params {
                                div { class: "py-3",
                                    div { class: "flex items-center gap-2",
                                        code { class: "text-sm font-mono font-medium text-foreground",
                                            {name.clone()}
                                        }
                                        span { class: "text-xs text-foreground-muted",
                                            "string"
                                        }
                                        span { class: "text-xs text-warning font-medium",
                                            {t.api_docs_required_label}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                if !endpoint.query_params.is_empty() {
                    div { class: "mt-4",
                        h4 { class: "text-xs font-semibold text-foreground-muted uppercase tracking-wider mb-3",
                            {t.api_docs_query_params}
                        }
                        div { class: "divide-y divide-border border-t border-border",
                            for (name , ty , is_optional) in &endpoint.query_params {
                                div { class: "py-3",
                                    div { class: "flex items-center gap-2",
                                        code { class: "text-sm font-mono font-medium text-foreground",
                                            {name.clone()}
                                        }
                                        span { class: "text-xs text-foreground-muted",
                                            {rust_type_simple(ty)}
                                        }
                                        if *is_optional {
                                            span { class: "text-xs text-foreground-muted italic",
                                                {t.api_docs_optional_label}
                                            }
                                        } else {
                                            span { class: "text-xs text-warning font-medium",
                                                {t.api_docs_required_label}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                if !endpoint.body_schema.is_empty() {
                    div { class: "mt-6",
                        h4 { class: "text-xs font-semibold text-foreground-muted uppercase tracking-wider mb-3",
                            {t.api_docs_body_fields}
                        }
                        div { class: "divide-y divide-border border-t border-border",
                            for field in &endpoint.body_schema {
                                FieldItem { field: field.clone(), ko }
                            }
                        }
                    }
                }

                if !endpoint.response_schema.is_empty() {
                    div { class: "mt-6",
                        h4 { class: "text-xs font-semibold text-foreground-muted uppercase tracking-wider mb-1",
                            {t.api_docs_returns}
                        }
                        p { class: "text-sm text-foreground-muted mb-3",
                            code { class: "text-xs font-mono bg-panel-strong px-1 py-0.5 rounded",
                                {endpoint.response_type.clone()}
                            }
                        }
                        div { class: "divide-y divide-border border-t border-border",
                            for field in &endpoint.response_schema {
                                ResponseFieldItem { field: field.clone(), ko }
                            }
                        }
                    }
                }
            }

            // ── RIGHT: code examples (always dark, sticky) ──
            div { class: "bg-[#0a0e1a] lg:sticky lg:top-14 lg:h-[calc(100vh-3.5rem)] lg:overflow-y-auto",
                div { class: "border-b border-gray-800",
                    div { class: "px-5 py-2.5 flex items-center justify-between",
                        span { class: "text-xs text-gray-400 font-medium uppercase tracking-wide",
                            {t.api_docs_request_label}
                        }
                        span { class: "text-xs text-gray-600", "curl" }
                    }
                    pre { class: "px-5 pb-5 text-[13px] font-mono text-gray-300 leading-relaxed overflow-x-auto",
                        {curl}
                    }
                }
                div {
                    div { class: "px-5 py-2.5 flex items-center justify-between",
                        span { class: "text-xs text-gray-400 font-medium uppercase tracking-wide",
                            {t.api_docs_response_label}
                        }
                        span { class: "text-xs text-gray-600", "JSON" }
                    }
                    div { class: "px-5 pb-5 max-h-[60vh] overflow-auto",
                        code { class: "block text-[13px] font-mono text-green-400 leading-relaxed whitespace-pre",
                            {response_json}
                        }
                    }
                }
            }
        }
    }
}

fn rust_type_simple(ty: &str) -> String {
    match ty {
        "String" | "ProjectPartition" => "string".to_string(),
        "i32" | "i64" => "integer".to_string(),
        "bool" => "boolean".to_string(),
        _ if ty.starts_with("Option<") => {
            let inner = &ty[7..ty.len() - 1];
            rust_type_simple(inner)
        }
        _ => ty.to_string(),
    }
}

#[component]
fn FieldItem(field: SchemaField, ko: bool) -> Element {
    let t: ApiDocsTranslate = use_translate();
    let desc = if ko && !field.description_ko.is_empty() {
        field.description_ko.clone()
    } else {
        field.description.clone()
    };

    rsx! {
        div { class: "py-3",
            if !field.name.is_empty() && !field.name.starts_with("__") {
                div { class: "flex items-center gap-2",
                    code { class: "text-sm font-mono font-medium text-foreground", {field.name.clone()} }
                    span { class: "text-xs text-foreground-muted", {field.field_type.clone()} }
                    if field.required {
                        span { class: "text-xs text-warning font-medium", {t.api_docs_required_label} }
                    } else {
                        span { class: "text-xs text-foreground-muted italic", {t.api_docs_optional_label} }
                    }
                }
            }
            if !field.enum_variants.is_empty() {
                EnumBadges { variants: field.enum_variants.clone(), ko }
            }
            if !desc.is_empty() {
                p { class: "text-sm text-foreground-muted mt-1", {desc} }
            }
            if !field.sub_fields.is_empty() {
                SubFields { fields: field.sub_fields.clone(), ko, is_response: false }
            }
        }
    }
}

#[component]
fn ResponseFieldItem(field: SchemaField, ko: bool) -> Element {
    let t: ApiDocsTranslate = use_translate();
    let desc = if ko && !field.description_ko.is_empty() {
        field.description_ko.clone()
    } else {
        field.description.clone()
    };

    rsx! {
        div { class: "py-3",
            if !field.name.is_empty() && !field.name.starts_with("__") {
                div { class: "flex items-center gap-2",
                    code { class: "text-sm font-mono font-medium text-foreground", {field.name.clone()} }
                    span { class: "text-xs text-foreground-muted", {field.field_type.clone()} }
                    if !field.required {
                        span { class: "text-xs text-foreground-muted italic", {t.api_docs_optional_label} }
                    }
                }
            }
            if !field.enum_variants.is_empty() {
                EnumBadges { variants: field.enum_variants.clone(), ko }
            }
            if !desc.is_empty() {
                p { class: "text-sm text-foreground-muted mt-1", {desc} }
            }
            if !field.sub_fields.is_empty() {
                SubFields { fields: field.sub_fields.clone(), ko, is_response: true }
            }
        }
    }
}

#[component]
fn EnumBadges(variants: Vec<EnumVariantInfo>, ko: bool) -> Element {
    let has_fields = variants.iter().any(|v| !v.fields.is_empty());

    if !has_fields {
        return rsx! {
            div { class: "flex flex-wrap gap-1 mt-1",
                for v in &variants {
                    code { class: "text-xs font-mono px-1.5 py-0.5 rounded bg-panel-strong text-foreground-muted",
                        {v.name.clone()}
                    }
                }
            }
        };
    }

    rsx! {
        div { class: "mt-2 space-y-2",
            for v in &variants {
                div { class: "rounded-lg border border-border bg-panel-muted/50 px-3 py-2",
                    if !v.fields.is_empty() {
                        div { class: "divide-y divide-border",
                            for f in &v.fields {
                                div { class: "py-1.5",
                                    if f.field_type.starts_with('"') {
                                        div { class: "flex items-center gap-2",
                                            code { class: "text-xs font-mono text-foreground",
                                                {f.name.clone()}
                                            }
                                            code { class: "text-xs font-mono px-1.5 py-0.5 rounded bg-brand-soft text-brand font-semibold",
                                                {f.field_type.clone()}
                                            }
                                        }
                                    } else {
                                        div { class: "flex items-center gap-2",
                                            code { class: "text-xs font-mono text-foreground-muted",
                                                {f.name.clone()}
                                            }
                                            span { class: "text-xs text-foreground-muted/70",
                                                {f.field_type.clone()}
                                            }
                                        }
                                        {
                                            let desc = if ko && !f.description_ko.is_empty() {
                                                f.description_ko.clone()
                                            } else {
                                                f.description.clone()
                                            };
                                            if !desc.is_empty() {
                                                rsx! { p { class: "text-xs text-foreground-muted mt-0.5", {desc} } }
                                            } else {
                                                rsx! {}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SubFields(fields: Vec<SchemaField>, ko: bool, is_response: bool) -> Element {
    let t: ApiDocsTranslate = use_translate();

    rsx! {
        div { class: "mt-2 ml-4 pl-3 border-l-2 border-border",
            div { class: "divide-y divide-border",
                for field in &fields {
                    div { class: "py-2",
                        div { class: "flex items-center gap-2",
                            code { class: "text-xs font-mono font-medium text-foreground", {field.name.clone()} }
                            span { class: "text-xs text-foreground-muted", {field.field_type.clone()} }
                            if !is_response && field.required {
                                span { class: "text-xs text-warning font-medium", {t.api_docs_required_label} }
                            }
                            if !field.required {
                                span { class: "text-xs text-foreground-muted italic", {t.api_docs_optional_label} }
                            }
                        }
                        {
                            let desc = if ko && !field.description_ko.is_empty() {
                                field.description_ko.clone()
                            } else {
                                field.description.clone()
                            };
                            if !desc.is_empty() {
                                rsx! { p { class: "text-xs text-foreground-muted mt-0.5", {desc} } }
                            } else {
                                rsx! {}
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn LanguageSwitcher() -> Element {
    let lang = dioxus_translate::use_language();
    let current = lang();
    let (current_label, next_label) = match current {
        dioxus_translate::Language::Ko => ("한국어", "EN"),
        _ => ("English", "KO"),
    };

    let on_toggle = move |_| {
        let new_lang = match lang() {
            dioxus_translate::Language::Ko => "en",
            _ => "ko",
        };
        document::eval(&format!(
            r#"localStorage.setItem("language", "{new_lang}");
            document.cookie = "language={new_lang}; path=/; max-age=31536000; samesite=lax";
            window.location.reload();"#
        ));
    };

    rsx! {
        button {
            onclick: on_toggle,
            class: "w-full flex items-center gap-2 px-2 py-2 text-sm text-foreground-muted hover:text-foreground hover:bg-background-muted rounded transition-colors",
            svg {
                class: "w-4 h-4 shrink-0",
                fill: "none",
                view_box: "0 0 24 24",
                stroke_width: "1.5",
                stroke: "currentColor",
                path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    d: "M10.5 21l5.25-11.25L21 21m-9-3h7.5M3 5.621a48.474 48.474 0 016-.371m0 0c1.12 0 2.233.038 3.334.114M9 5.25V3m3.334 2.364C11.176 10.658 7.69 15.08 3 17.502m9.334-12.138c.896.061 1.785.147 2.666.257m-4.589 8.495a18.023 18.023 0 01-3.827-5.802",
                }
            }
            span { {current_label} }
            span { class: "ml-auto text-xs text-foreground-muted/60", "→ {next_label}" }
        }
    }
}
