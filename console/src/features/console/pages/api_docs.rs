use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SchemaField {
    pub name: String,
    pub field_type: String,
    pub required: bool,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EndpointInfo {
    pub method: String,
    pub path: String,
    pub group: String,
    pub summary: String,
    pub description: String,
    pub auth: String,
    pub path_params: Vec<(String, String)>,
    pub query_params: Vec<(String, String, bool)>,
    pub body_params: Vec<(String, String)>,
    pub response_type: String,
    pub body_schema: Vec<SchemaField>,
    pub response_schema: Vec<SchemaField>,
}

#[cfg(feature = "server")]
use crate::common::types::api_doc_meta;

#[server]
async fn get_api_endpoints() -> Result<Vec<EndpointInfo>, ServerFnError> {
    let mut endpoints: Vec<EndpointInfo> = api_doc_meta::all_api_docs()
        .into_iter()
        .map(|ep| {
            let body_type = ep
                .body_params
                .first()
                .map(|(_, t)| t.to_string())
                .unwrap_or_default();
            let body_schema = extract_schema_fields(&body_type);
            let response_schema = extract_schema_fields(ep.response_type);

            EndpointInfo {
                method: ep.method.to_string(),
                path: ep.path.to_string(),
                group: ep.group.to_string(),
                summary: ep.summary.to_string(),
                description: ep.description.to_string(),
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

    endpoints.sort_by(|a, b| a.group.cmp(&b.group).then(a.path.cmp(&b.path)));
    Ok(endpoints)
}

#[cfg(feature = "server")]
fn extract_schema_fields(type_name: &str) -> Vec<SchemaField> {
    let schema_json = match api_doc_meta::schema_for_type(type_name) {
        Some(v) => v,
        None => return vec![],
    };

    let mut fields = vec![];
    let required_fields: Vec<String> = schema_json
        .get("required")
        .and_then(|r| r.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    if let Some(properties) = schema_json.get("properties").and_then(|p| p.as_object()) {
        for (name, prop) in properties {
            let field_type = json_schema_type_to_string(prop);
            let description = prop
                .get("description")
                .and_then(|d| d.as_str())
                .unwrap_or("")
                .to_string();
            fields.push(SchemaField {
                name: name.clone(),
                field_type,
                required: required_fields.contains(name),
                description,
            });
        }
    }

    if fields.is_empty() {
        if let Some(one_of) = schema_json.get("oneOf").and_then(|o| o.as_array()) {
            for variant in one_of {
                if let Some(props) = variant.get("properties").and_then(|p| p.as_object()) {
                    for (name, prop) in props {
                        let field_type = json_schema_type_to_string(prop);
                        let description = prop
                            .get("description")
                            .and_then(|d| d.as_str())
                            .unwrap_or("")
                            .to_string();
                        if !fields.iter().any(|f: &SchemaField| f.name == *name) {
                            fields.push(SchemaField {
                                name: name.clone(),
                                field_type,
                                required: false,
                                description,
                            });
                        }
                    }
                }
            }
        }
    }

    fields.sort_by(|a, b| b.required.cmp(&a.required).then(a.name.cmp(&b.name)));
    fields
}

#[cfg(feature = "server")]
fn json_schema_type_to_string(prop: &serde_json::Value) -> String {
    if let Some(ty) = prop.get("type").and_then(|t| t.as_str()) {
        match ty {
            "string" => "string".to_string(),
            "integer" => {
                let fmt = prop
                    .get("format")
                    .and_then(|f| f.as_str())
                    .unwrap_or("int64");
                fmt.to_string()
            }
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
        }
    } else if prop.get("oneOf").is_some() || prop.get("anyOf").is_some() {
        "enum".to_string()
    } else if let Some(r) = prop.get("$ref").and_then(|r| r.as_str()) {
        r.rsplit('/').next().unwrap_or("object").to_string()
    } else {
        "any".to_string()
    }
}

// ── Page ───────────────────────────────────────────────────────────

#[component]
pub fn ApiDocs() -> Element {
    let endpoints = use_server_future(get_api_endpoints)?;

    let endpoints = match endpoints() {
        Some(Ok(eps)) => eps,
        Some(Err(_)) => return rsx! { div { class: "p-8", "Failed to load API documentation." } },
        None => return rsx! { div { class: "p-8", "Loading..." } },
    };

    let mut groups: Vec<(String, Vec<&EndpointInfo>)> = Vec::new();
    for ep in &endpoints {
        if let Some(g) = groups.iter_mut().find(|(name, _)| name == &ep.group) {
            g.1.push(ep);
        } else {
            groups.push((ep.group.clone(), vec![ep]));
        }
    }

    rsx! {
        div { class: "min-h-screen bg-white text-gray-900",
            // ── Sticky header ──
            header { class: "border-b border-gray-200 bg-white/95 backdrop-blur sticky top-0 z-20",
                div { class: "max-w-screen-2xl mx-auto px-6 h-14 flex items-center justify-between",
                    div { class: "flex items-center gap-3",
                        span { class: "font-semibold text-lg tracking-tight", "Biyard" }
                        span { class: "text-gray-300", "/" }
                        span { class: "text-gray-500 text-sm", "API Reference" }
                    }
                    a {
                        href: "/docs/api.json",
                        target: "_blank",
                        class: "text-sm text-indigo-600 hover:text-indigo-800",
                        "OpenAPI JSON"
                    }
                }
            }

            // ── Two-column shell ──
            div { class: "max-w-screen-2xl mx-auto flex",
                // Left sidebar nav
                nav { class: "hidden lg:block w-56 shrink-0 border-r border-gray-100 sticky top-14 h-[calc(100vh-3.5rem)] overflow-y-auto py-6 px-4",
                    for (group_name, group_eps) in &groups {
                        div { class: "mb-5",
                            h3 { class: "text-xs font-semibold text-gray-400 uppercase tracking-wider mb-2 px-2",
                                {group_name.clone()}
                            }
                            for ep in group_eps {
                                a {
                                    href: "#ep-{ep.method}-{ep.path}",
                                    class: "block px-2 py-1.5 text-sm text-gray-600 hover:text-gray-900 hover:bg-gray-50 rounded truncate",
                                    title: "{ep.path}",
                                    span { class: "font-mono text-xs mr-1.5 {method_text_color(&ep.method)}",
                                        {ep.method.clone()}
                                    }
                                    {ep.summary.clone()}
                                }
                            }
                        }
                    }
                }

                // Main content
                main { class: "flex-1 min-w-0",
                    // ── Hero / Auth section ──
                    div { class: "border-b border-gray-100 py-10 px-6 lg:px-10",
                        div { class: "grid lg:grid-cols-2 gap-8 max-w-5xl",
                            // Left: description
                            div {
                                h1 { class: "text-3xl font-bold mb-4", "Biyard API" }
                                p { class: "text-gray-500 leading-relaxed mb-6",
                                    "Manage points and tokens for your project on blockchain. "
                                    "Award points, mint tokens, and query balances programmatically via REST."
                                }
                                div { class: "flex items-center gap-2 text-sm",
                                    span { class: "text-gray-400", "Base URL" }
                                    code { class: "font-mono text-sm bg-gray-100 px-2 py-1 rounded",
                                        "https://api.biyard.co"
                                    }
                                }
                            }
                            // Right: auth code block
                            div { class: "bg-gray-950 rounded-xl overflow-hidden",
                                div { class: "px-4 py-2.5 border-b border-gray-800 flex items-center gap-2",
                                    span { class: "text-xs text-gray-400 font-medium", "Authentication" }
                                }
                                pre { class: "px-4 py-4 text-sm font-mono leading-relaxed overflow-x-auto",
                                    span { class: "text-gray-500", "curl https://api.biyard.co/v1/... \\\n" }
                                    span { class: "text-gray-500", "  -H " }
                                    span { class: "text-green-400", "\"Authorization: Bearer " }
                                    span { class: "text-yellow-300 italic", "sk_live_..." }
                                    span { class: "text-green-400", "\"" }
                                }
                            }
                        }
                    }

                    // ── Endpoints ──
                    for (group_name, group_endpoints) in &groups {
                        div { class: "border-b border-gray-100",
                            div { class: "px-6 lg:px-10 pt-10 pb-2",
                                h2 { class: "text-xl font-bold", {group_name.clone()} }
                            }
                            for ep in group_endpoints {
                                EndpointSection { endpoint: (*ep).clone() }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn method_text_color(method: &str) -> &'static str {
    match method {
        "GET" => "text-green-600",
        "POST" => "text-blue-600",
        "PUT" => "text-amber-600",
        "PATCH" => "text-orange-600",
        "DELETE" => "text-red-600",
        _ => "text-gray-600",
    }
}

fn method_bg(method: &str) -> &'static str {
    match method {
        "GET" => "bg-green-500",
        "POST" => "bg-blue-500",
        "PUT" => "bg-amber-500",
        "PATCH" => "bg-orange-500",
        "DELETE" => "bg-red-500",
        _ => "bg-gray-500",
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
        .replace(":project_id", "proj_abc123")
        .replace(":meta_user_id", "user_xyz");

    let mut curl = format!("curl https://api.biyard.co{}", api_path);

    if ep.method != "GET" {
        curl = format!("curl -X {} https://api.biyard.co{}", ep.method, api_path);
    }

    curl.push_str(" \\\n  -H \"Authorization: Bearer sk_live_...\"");

    if !ep.body_params.is_empty() || !ep.body_schema.is_empty() {
        curl.push_str(" \\\n  -H \"Content-Type: application/json\"");
        curl.push_str(" \\\n  -d '");

        if !ep.body_schema.is_empty() {
            let json_fields: Vec<String> = ep
                .body_schema
                .iter()
                .filter(|f| f.required)
                .map(|f| {
                    let val = match f.field_type.as_str() {
                        "string" => "\"...\"".to_string(),
                        "int64" | "int32" | "uint16" | "uint64" => "100".to_string(),
                        "number" => "1.0".to_string(),
                        "boolean" => "true".to_string(),
                        _ if f.field_type.ends_with("[]") => "[...]".to_string(),
                        _ => "\"...\"".to_string(),
                    };
                    format!("  \"{}\": {}", f.name, val)
                })
                .collect();

            if json_fields.is_empty() {
                curl.push_str("{...}");
            } else {
                curl.push_str("{\n");
                curl.push_str(&json_fields.join(",\n"));
                curl.push_str("\n}");
            }
        } else {
            curl.push_str("{...}");
        }
        curl.push('\'');
    }

    curl
}

fn build_response_example(ep: &EndpointInfo) -> String {
    if ep.response_schema.is_empty() {
        return "{}".to_string();
    }

    let fields: Vec<String> = ep
        .response_schema
        .iter()
        .take(8)
        .map(|f| {
            let val = match f.field_type.as_str() {
                "string" => "\"...\"".to_string(),
                "int64" | "int32" | "uint16" | "uint64" => "0".to_string(),
                "number" => "0.0".to_string(),
                "boolean" => "false".to_string(),
                _ if f.field_type.ends_with("[]") => "[]".to_string(),
                _ => "null".to_string(),
            };
            format!("  \"{}\": {}", f.name, val)
        })
        .collect();

    let mut json = "{\n".to_string();
    json.push_str(&fields.join(",\n"));
    if ep.response_schema.len() > 8 {
        json.push_str(",\n  ...");
    }
    json.push_str("\n}");
    json
}

// ── Stripe-style endpoint section (two-column) ────────────────────

#[component]
fn EndpointSection(endpoint: EndpointInfo) -> Element {
    let curl = build_curl_example(&endpoint);
    let response_json = build_response_example(&endpoint);
    let permission = auth_to_permission(&endpoint.auth);
    let method_dot = method_bg(&endpoint.method);

    let all_params: Vec<ParamRow> = build_param_rows(&endpoint);

    rsx! {
        div {
            id: "ep-{endpoint.method}-{endpoint.path}",
            class: "grid lg:grid-cols-2 border-t border-gray-100",

            // ── LEFT: description + parameters ──
            div { class: "px-6 lg:px-10 py-8 lg:border-r border-gray-100",
                // Title
                div { class: "flex items-center gap-2.5 mb-1",
                    span { class: "w-2 h-2 rounded-full {method_dot}" }
                    h3 { class: "font-semibold text-base", {endpoint.summary.clone()} }
                }
                div { class: "flex items-center gap-2 mb-4",
                    code { class: "text-xs font-mono text-gray-500",
                        "{endpoint.method} {endpoint.path}"
                    }
                    span { class: "text-xs text-gray-400 border border-gray-200 rounded px-1.5 py-0.5",
                        {permission}
                    }
                }

                if !endpoint.description.is_empty() {
                    p { class: "text-sm text-gray-500 leading-relaxed mb-6",
                        {endpoint.description.clone()}
                    }
                }

                // Parameters list (Stripe style)
                if !all_params.is_empty() {
                    div { class: "mt-4",
                        h4 { class: "text-xs font-semibold text-gray-400 uppercase tracking-wider mb-3",
                            "Parameters"
                        }
                        div { class: "divide-y divide-gray-100 border-t border-gray-100",
                            for param in &all_params {
                                ParamItem { param: param.clone() }
                            }
                        }
                    }
                }

                // Body schema fields
                if !endpoint.body_schema.is_empty() {
                    div { class: "mt-6",
                        h4 { class: "text-xs font-semibold text-gray-400 uppercase tracking-wider mb-3",
                            "Body fields"
                        }
                        div { class: "divide-y divide-gray-100 border-t border-gray-100",
                            for field in &endpoint.body_schema {
                                FieldItem { field: field.clone() }
                            }
                        }
                    }
                }

                // Response fields
                if !endpoint.response_schema.is_empty() {
                    div { class: "mt-6",
                        h4 { class: "text-xs font-semibold text-gray-400 uppercase tracking-wider mb-1",
                            "Returns"
                        }
                        p { class: "text-sm text-gray-500 mb-3",
                            code { class: "text-xs font-mono bg-gray-100 px-1 py-0.5 rounded",
                                {endpoint.response_type.clone()}
                            }
                        }
                        div { class: "divide-y divide-gray-100 border-t border-gray-100",
                            for field in &endpoint.response_schema {
                                FieldItem { field: field.clone() }
                            }
                        }
                    }
                }
            }

            // ── RIGHT: code examples (dark panel, sticky) ──
            div { class: "bg-gray-950 lg:sticky lg:top-14 lg:h-[calc(100vh-3.5rem)] lg:overflow-y-auto",
                // Request
                div { class: "border-b border-gray-800",
                    div { class: "px-5 py-2.5 flex items-center justify-between",
                        span { class: "text-xs text-gray-400 font-medium uppercase tracking-wide", "Request" }
                        span { class: "text-xs text-gray-600", "curl" }
                    }
                    pre { class: "px-5 pb-5 text-[13px] font-mono text-gray-300 leading-relaxed overflow-x-auto",
                        {curl}
                    }
                }
                // Response
                div {
                    div { class: "px-5 py-2.5 flex items-center justify-between",
                        span { class: "text-xs text-gray-400 font-medium uppercase tracking-wide", "Response" }
                        span { class: "text-xs text-gray-600", "JSON" }
                    }
                    pre { class: "px-5 pb-5 text-[13px] font-mono text-green-400 leading-relaxed overflow-x-auto",
                        {response_json}
                    }
                }
            }
        }
    }
}

// ── Parameter row (Stripe style) ──────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
struct ParamRow {
    name: String,
    location: String,
    param_type: String,
    required: bool,
}

fn build_param_rows(ep: &EndpointInfo) -> Vec<ParamRow> {
    let mut rows = vec![];

    for (name, _ty) in &ep.path_params {
        rows.push(ParamRow {
            name: name.clone(),
            location: "path".to_string(),
            param_type: "string".to_string(),
            required: true,
        });
    }
    for (name, ty, is_optional) in &ep.query_params {
        rows.push(ParamRow {
            name: name.clone(),
            location: "query".to_string(),
            param_type: rust_type_simple(ty),
            required: !is_optional,
        });
    }

    rows
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
fn ParamItem(param: ParamRow) -> Element {
    rsx! {
        div { class: "py-3",
            div { class: "flex items-center gap-2 mb-0.5",
                code { class: "text-sm font-mono font-medium text-gray-900", {param.name.clone()} }
                span { class: "text-xs text-gray-400", {param.param_type.clone()} }
                if param.required {
                    span { class: "text-xs text-orange-500 font-medium", "REQUIRED" }
                }
            }
            p { class: "text-xs text-gray-400",
                {format!("In {}", param.location)}
            }
        }
    }
}

#[component]
fn FieldItem(field: SchemaField) -> Element {
    rsx! {
        div { class: "py-3",
            div { class: "flex items-center gap-2",
                code { class: "text-sm font-mono font-medium text-gray-900",
                    {field.name.clone()}
                }
                span { class: "text-xs text-gray-400", {field.field_type.clone()} }
                if field.required {
                    span { class: "text-xs text-orange-500 font-medium", "REQUIRED" }
                }
            }
            if !field.description.is_empty() {
                p { class: "text-sm text-gray-500 mt-1", {field.description.clone()} }
            }
        }
    }
}
