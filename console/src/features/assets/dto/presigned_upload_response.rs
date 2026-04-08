use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PresignedUploadResponse {
    /// PUT this URL with the file body.
    pub presigned_url: String,
    /// Public CloudFront URL the object will be served from after upload.
    pub public_url: String,
    /// Object key inside the bucket.
    pub key: String,
}
