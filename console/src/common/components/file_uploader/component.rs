use dioxus::html::{FileData, HasFileData};
use dioxus::prelude::*;

use crate::common::Result;

/// Metadata for an uploaded file, surfaced to the parent component via the
/// optional `on_upload_meta` callback.
#[derive(Clone, Debug, PartialEq)]
pub struct UploadedFileMeta {
    /// Public CloudFront URL the object is served from.
    pub url: String,
    /// Original filename selected by the user.
    pub name: String,
    /// Human-readable size, e.g. "2.4 MB".
    pub size: String,
}

/// A drag-and-drop file uploader that:
///
/// 1. Asks the server for a presigned PUT URL via `create_presigned_upload_handler`
/// 2. PUTs the file body directly to S3
/// 3. Calls `on_upload_success` with the public CloudFront URL
///
/// `prefix` controls the S3 key namespace, e.g. `"projects/<id>/logos"`.
#[component]
pub fn FileUploader(
    on_upload_success: EventHandler<String>,
    #[props(default)] on_upload_meta: Option<EventHandler<UploadedFileMeta>>,
    #[props(default)] prefix: Option<String>,
    #[props(default)] class: Option<String>,
    #[props(default)] accept: Option<String>,
    children: Element,
) -> Element {
    let accept = accept.unwrap_or_else(|| "image/*".to_string());
    let class_name = class
        .map(|c| format!("cursor-pointer {}", c))
        .unwrap_or_else(|| "cursor-pointer".to_string());

    let start_upload = {
        let on_upload_success = on_upload_success;
        let on_upload_meta = on_upload_meta;
        let prefix = prefix.clone();
        move |file: FileData| {
            let prefix = prefix.clone();
            spawn(async move {
                if let Err(err) =
                    upload_via_presigned(prefix, file, on_upload_success, on_upload_meta).await
                {
                    tracing::error!("FileUploader upload failed: {err:?}");
                }
            });
        }
    };

    let on_change = {
        let start_upload = start_upload.clone();
        move |evt: FormEvent| {
            let Some(file) = evt.files().into_iter().next() else {
                return;
            };
            start_upload(file);
            // Reset the input value so the same file can be re-selected later.
            #[cfg(feature = "web")]
            {
                use dioxus::web::WebEventExt;
                use wasm_bindgen::JsCast;
                if let Some(web_event) = evt.try_as_web_event() {
                    if let Some(input) = web_event
                        .target()
                        .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                    {
                        input.set_value("");
                    }
                }
            }
        }
    };

    let on_drop = {
        let start_upload = start_upload.clone();
        move |evt: DragEvent| {
            evt.prevent_default();
            let Some(file) = evt.files().into_iter().next() else {
                return;
            };
            start_upload(file);
        }
    };

    let on_drag_over = move |evt: DragEvent| {
        evt.prevent_default();
    };

    rsx! {
        label {
            class: "{class_name}",
            ondragover: on_drag_over,
            ondrop: on_drop,
            input {
                class: "hidden",
                r#type: "file",
                accept: "{accept}",
                onchange: on_change,
            }
            {children}
        }
    }
}

#[cfg(not(feature = "web"))]
async fn upload_via_presigned(
    _prefix: Option<String>,
    _file: FileData,
    _on_upload_success: EventHandler<String>,
    _on_upload_meta: Option<EventHandler<UploadedFileMeta>>,
) -> Result<()> {
    Ok(())
}

#[cfg(feature = "web")]
async fn upload_via_presigned(
    prefix: Option<String>,
    file: FileData,
    on_upload_success: EventHandler<String>,
    on_upload_meta: Option<EventHandler<UploadedFileMeta>>,
) -> Result<()> {
    use crate::common::Error;
    use dioxus::web::WebFileExt;
    use wasm_bindgen::JsCast;

    let file_name = file.name();
    let Some(web_file) = file.get_web_file() else {
        return Err(Error::NotFound("Failed to get web file".to_string()));
    };

    if web_file.size() > 100_f64 * 1024_f64 * 1024_f64 {
        return Err(Error::BadRequest(
            "Files larger than 100MB are not supported.".to_string(),
        ));
    }

    let presigned =
        crate::features::assets::controllers::create_presigned_upload_handler(prefix).await?;

    let content_type = web_file.type_();
    let size = format_file_size(web_file.size());

    let opts = web_sys::RequestInit::new();
    opts.set_method("PUT");
    let body = wasm_bindgen::JsValue::from(web_file);
    opts.set_body(&body);

    let request = web_sys::Request::new_with_str_and_init(&presigned.presigned_url, &opts)
        .map_err(|e| Error::Unknown(js_error_to_string(e)))?;
    if !content_type.is_empty() {
        request
            .headers()
            .set("Content-Type", &content_type)
            .map_err(|e| Error::Unknown(js_error_to_string(e)))?;
    }

    let window =
        web_sys::window().ok_or_else(|| Error::NotFound("No window available.".to_string()))?;
    let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| Error::Unknown(js_error_to_string(e)))?;
    let resp: web_sys::Response = resp_value
        .dyn_into()
        .map_err(|_| Error::Unknown("Invalid upload response.".to_string()))?;
    if !resp.ok() {
        return Err(Error::InternalServerError(format!(
            "Upload failed ({})",
            resp.status()
        )));
    }

    on_upload_success.call(presigned.public_url.clone());
    if let Some(on_upload_meta) = on_upload_meta {
        on_upload_meta.call(UploadedFileMeta {
            url: presigned.public_url,
            name: file_name,
            size,
        });
    }
    Ok(())
}

#[cfg(feature = "web")]
fn format_file_size(size_bytes: f64) -> String {
    let mb = size_bytes / (1024_f64 * 1024_f64);
    if mb >= 1_f64 {
        format!("{:.1} MB", mb)
    } else {
        let kb = size_bytes / 1024_f64;
        format!("{:.1} KB", kb)
    }
}

#[cfg(feature = "web")]
fn js_error_to_string(err: wasm_bindgen::JsValue) -> String {
    err.as_string()
        .unwrap_or_else(|| "Unknown error".to_string())
}
