use aws_sdk_s3::{Client, presigning::PresigningConfig};
use uuid::Uuid;

use crate::common::{Error, Result};

/// Thin wrapper around `aws-sdk-s3` that handles presigned PUT URL generation
/// and converts S3 keys into public CloudFront URLs.
#[derive(Debug, Clone)]
pub struct S3Client {
    client: Client,
    bucket_name: String,
    /// Public URL prefix (e.g., "https://meta.biyard.co") used by `get_url`.
    public_url: String,
    /// Default presigned URL expiration in seconds.
    expire: u64,
}

#[derive(Debug, Clone)]
pub struct PresignedUpload {
    /// Presigned PUT URL the browser uses to upload directly to S3.
    pub presigned_uri: String,
    /// Public CloudFront URL the uploaded object will be served from.
    pub public_url: String,
    /// Object key inside the bucket.
    pub key: String,
}

impl S3Client {
    pub fn new(client: Client, bucket_name: String, public_url: String, expire: u64) -> Self {
        let public_url = public_url.trim_end_matches('/').to_string();
        Self {
            client,
            bucket_name,
            public_url,
            expire,
        }
    }

    /// Build a public CloudFront URL for an existing key.
    pub fn get_url(&self, key: &str) -> String {
        format!("{}/{}", self.public_url, key.trim_start_matches('/'))
    }

    /// Generate a presigned PUT URL.
    ///
    /// The final key is `<prefix>/<uuid>` (or just `<uuid>` if no prefix is
    /// provided). Browsers PUT the file body directly to `presigned_uri`,
    /// then the caller stores `public_url` in DynamoDB.
    pub async fn presign_upload(&self, prefix: Option<&str>) -> Result<PresignedUpload> {
        let id = Uuid::now_v7();
        let key = match prefix {
            Some(p) if !p.is_empty() => format!("{}/{}", p.trim_matches('/'), id),
            _ => id.to_string(),
        };

        let presigning_config = PresigningConfig::expires_in(std::time::Duration::from_secs(
            self.expire,
        ))
        .map_err(|e| {
            tracing::error!("invalid presign expire: {}", e);
            Error::InternalServerError(e.to_string())
        })?;

        let presigned = self
            .client
            .put_object()
            .bucket(&self.bucket_name)
            .key(&key)
            .presigned(presigning_config)
            .await
            .map_err(|e| {
                tracing::error!("failed to presign put_object: {}", e);
                Error::InternalServerError(e.to_string())
            })?;

        Ok(PresignedUpload {
            presigned_uri: presigned.uri().to_string(),
            public_url: self.get_url(&key),
            key,
        })
    }
}
