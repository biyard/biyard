//! HTTP driver for integration tests (Ratel-style oneshot).
//!
//! Dispatches requests against an in-memory `axum::Router` via
//! `tower::ServiceExt::oneshot`. A per-`Client` cookie store mirrors a
//! browser cookie jar so signin → subsequent request threads the
//! `local_sid` cookie automatically. Bearer tokens are pinned per call.

use axum::body::{Body, HttpBody};
use axum::http::header::{AUTHORIZATION, CONTENT_TYPE, COOKIE, HeaderValue, SET_COOKIE};
use axum::http::{HeaderMap, Method, Request, StatusCode};
use serde::Serialize;
use std::sync::{Arc, Mutex};
use tower::ServiceExt;

#[derive(Clone)]
pub struct Client {
    router: axum::Router,
    cookies: Arc<Mutex<Vec<(String, String)>>>,
}

impl Client {
    pub fn new(router: axum::Router) -> Self {
        Self {
            router,
            cookies: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn get(&self, path: &str) -> Response {
        self.request(Method::GET, path, Body::empty(), None).await
    }

    pub async fn get_with_bearer(&self, path: &str, bearer: &str) -> Response {
        self.request(Method::GET, path, Body::empty(), Some(bearer))
            .await
    }

    pub async fn post_json<T: Serialize>(&self, path: &str, body: &T) -> Response {
        let bytes = serde_json::to_vec(body).expect("serialize body");
        self.request(Method::POST, path, Body::from(bytes), None)
            .await
    }

    pub async fn post_json_with_bearer<T: Serialize>(
        &self,
        path: &str,
        body: &T,
        bearer: &str,
    ) -> Response {
        let bytes = serde_json::to_vec(body).expect("serialize body");
        self.request(Method::POST, path, Body::from(bytes), Some(bearer))
            .await
    }

    pub async fn put_json<T: Serialize>(&self, path: &str, body: &T) -> Response {
        let bytes = serde_json::to_vec(body).expect("serialize body");
        self.request(Method::PUT, path, Body::from(bytes), None)
            .await
    }

    pub async fn delete(&self, path: &str) -> Response {
        self.request(Method::DELETE, path, Body::empty(), None)
            .await
    }

    async fn request(
        &self,
        method: Method,
        path: &str,
        body: Body,
        bearer: Option<&str>,
    ) -> Response {
        let mut builder = Request::builder().method(method).uri(path);

        if let Some(headers) = builder.headers_mut() {
            let size = body.size_hint().exact().unwrap_or_default();
            if size > 0 {
                headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
            }
            if let Some(token) = bearer {
                headers.insert(
                    AUTHORIZATION,
                    HeaderValue::from_str(&format!("Bearer {token}")).unwrap(),
                );
            }
            if let Some(cookie_header) = self.cookie_header() {
                headers.insert(COOKIE, cookie_header);
            }
        }

        let req = builder.body(body).expect("build request");
        let res = self
            .router
            .clone()
            .oneshot(req)
            .await
            .expect("router oneshot");

        let (parts, body) = res.into_parts();
        let bytes = axum::body::to_bytes(body, 10 * 1024 * 1024)
            .await
            .expect("read body");
        let body_text = String::from_utf8_lossy(&bytes).to_string();

        self.store_cookies(&parts.headers);

        Response {
            status: parts.status,
            body_text,
        }
    }

    fn cookie_header(&self) -> Option<HeaderValue> {
        let guard = self.cookies.lock().unwrap();
        if guard.is_empty() {
            return None;
        }
        let joined = guard
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("; ");
        HeaderValue::from_str(&joined).ok()
    }

    fn store_cookies(&self, headers: &HeaderMap) {
        let mut guard = self.cookies.lock().unwrap();
        for set_cookie in headers.get_all(SET_COOKIE).iter() {
            let Ok(raw) = set_cookie.to_str() else {
                continue;
            };
            // "name=value; Path=/; ..." — take the first attribute.
            let first = raw.split(';').next().unwrap_or("").trim();
            let Some((name, value)) = first.split_once('=') else {
                continue;
            };
            let name = name.trim().to_string();
            let value = value.trim().to_string();
            if let Some(existing) = guard.iter_mut().find(|(n, _)| n == &name) {
                existing.1 = value;
            } else {
                guard.push((name, value));
            }
        }
    }

    /// Perform the standard signin flow. Panics on failure.
    pub async fn signin(&self, email: &str, password: &str) {
        #[derive(Serialize)]
        struct Body<'a> {
            email: &'a str,
            password: &'a str,
        }
        let res = self
            .post_json("/v1/accounts/signin", &Body { email, password })
            .await;
        assert!(
            res.status.is_success(),
            "signin failed for {email}: {} — {}",
            res.status,
            res.body_text
        );
    }
}

pub struct Response {
    pub status: StatusCode,
    pub body_text: String,
}

impl Response {
    pub fn json<T: serde::de::DeserializeOwned>(&self) -> T {
        serde_json::from_str(&self.body_text).unwrap_or_else(|e| {
            panic!(
                "json decode failed: {e}\nstatus: {}\nbody: {}",
                self.status, self.body_text
            )
        })
    }
}
