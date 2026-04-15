use dioxus::fullstack::axum::{
    Json, Router,
    response::{Html, IntoResponse},
    routing::get,
};

const EXCHANGE_HTML: &str = include_str!("../../dapp/exchange.html");
const BUYBACK_HTML: &str = include_str!("../../dapp/buyback.html");

pub fn router() -> Router {
    Router::new()
        .route("/dapp/exchange", get(serve_exchange))
        .route("/dapp/buyback", get(serve_buyback))
        .route("/docs/api.json", get(serve_api_docs_json))
}

async fn serve_exchange() -> impl IntoResponse {
    Html(EXCHANGE_HTML)
}

async fn serve_buyback() -> impl IntoResponse {
    Html(BUYBACK_HTML)
}

async fn serve_api_docs_json() -> impl IntoResponse {
    let endpoints = crate::common::types::api_doc_meta::all_api_docs();
    let openapi = build_openapi_spec(&endpoints);
    Json(openapi)
}

fn build_openapi_spec(
    endpoints: &[&crate::common::types::api_doc_meta::ApiEndpointMeta],
) -> serde_json::Value {
    use serde_json::json;

    let mut paths = serde_json::Map::new();

    for ep in endpoints {
        let openapi_path = ep
            .path
            .split('?')
            .next()
            .unwrap_or(ep.path)
            .replace(":..", "{rest}")
            .split('/')
            .map(|seg| {
                if let Some(name) = seg.strip_prefix(':') {
                    format!("{{{name}}}")
                } else {
                    seg.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("/");

        let mut parameters = Vec::new();

        for (name, ty) in ep.path_params {
            parameters.push(json!({
                "name": name,
                "in": "path",
                "required": true,
                "schema": { "type": rust_type_to_openapi_type(ty) }
            }));
        }

        for (name, ty, is_optional) in ep.query_params {
            parameters.push(json!({
                "name": name,
                "in": "query",
                "required": !is_optional,
                "schema": { "type": rust_type_to_openapi_type(ty) }
            }));
        }

        let mut operation = json!({
            "summary": ep.summary,
            "tags": [ep.group],
            "responses": {
                "200": {
                    "description": "Success",
                    "content": {
                        "application/json": {
                            "schema": { "type": "object", "description": ep.response_type }
                        }
                    }
                }
            }
        });

        if !ep.description.is_empty() {
            operation["description"] = json!(ep.description);
        }

        if !ep.auth.is_empty() {
            operation["security"] = json!([{ "BearerAuth": [] }]);
        }

        if !parameters.is_empty() {
            operation["parameters"] = json!(parameters);
        }

        if !ep.body_params.is_empty() {
            let mut properties = serde_json::Map::new();
            let mut required = Vec::new();
            for (name, ty) in ep.body_params {
                properties.insert(
                    name.to_string(),
                    json!({ "type": rust_type_to_openapi_type(ty) }),
                );
                required.push(json!(name));
            }
            operation["requestBody"] = json!({
                "required": true,
                "content": {
                    "application/json": {
                        "schema": {
                            "type": "object",
                            "properties": properties,
                            "required": required,
                        }
                    }
                }
            });
        }

        let method = ep.method.to_lowercase();
        let path_entry = paths
            .entry(openapi_path)
            .or_insert_with(|| json!({}));
        path_entry[method] = operation;
    }

    json!({
        "openapi": "3.1.0",
        "info": {
            "title": "Biyard API",
            "version": "1.0.0",
            "description": "Biyard PaaS API for managing projects, points, and tokens on blockchain."
        },
        "servers": [
            { "url": "/", "description": "Current server" }
        ],
        "components": {
            "securitySchemes": {
                "BearerAuth": {
                    "type": "http",
                    "scheme": "bearer",
                    "description": "API key obtained from the Credentials endpoints"
                }
            }
        },
        "paths": paths,
    })
}

fn rust_type_to_openapi_type(ty: &str) -> &str {
    match ty {
        "String" | "ProjectPartition" | "Partition" => "string",
        "i32" | "i64" | "u16" | "u64" => "integer",
        "f64" | "f32" => "number",
        "bool" => "boolean",
        _ if ty.starts_with("Option<") => {
            let inner = &ty[7..ty.len() - 1];
            rust_type_to_openapi_type(inner)
        }
        _ if ty.starts_with("Vec<") => "array",
        _ => "object",
    }
}
