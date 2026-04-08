use crate::common::Result;
use crate::features::assets::PresignedUploadResponse;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::common::CommonConfig;
#[cfg(feature = "server")]
use crate::features::accounts::Account;

/// Generate a presigned PUT URL the browser can use to upload a file
/// directly to S3 (`meta.biyard.co`).
///
/// `prefix` controls the key namespace (e.g. `projects/<id>/logos`). The
/// final key is `<prefix>/<uuid>` so concurrent uploads never collide.
#[get("/v1/assets/presigned-upload?prefix", _account: Account)]
pub async fn create_presigned_upload_handler(
    prefix: Option<String>,
) -> Result<PresignedUploadResponse> {
    let config = CommonConfig::default();
    let s3 = config.s3();

    let upload = s3.presign_upload(prefix.as_deref()).await?;

    Ok(PresignedUploadResponse {
        presigned_url: upload.presigned_uri,
        public_url: upload.public_url,
        key: upload.key,
    })
}
