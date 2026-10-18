use crate::errors::{ServiceError, ServiceResult};
use rusoto_core::request::HttpClient;
use rusoto_s3::{CopyObjectRequest, GetObjectRequest, S3Client, S3};
use rusoto_signature::credential::StaticProvider;
use sha2::{Digest, Sha256};
use std::cmp;
use tokio::io::AsyncReadExt;

// const DERIVE_KEY: &str = "CADNICE";

impl From<&super::s3::Aws> for S3Client {
    /// Get S3Client from Aws data
    fn from(aws_access: &super::s3::Aws) -> S3Client {
        S3Client::new_with(
            HttpClient::new().expect("Failed to creat HTTP client"),
            StaticProvider::from(aws_access.clone_credentials()),
            aws_access.clone_region(),
        )
    }
}

/// Get object body from storage
pub(crate) async fn get_object_body(
    client: &S3Client,
    bucket: &str,
    path_file: &str,
    buffer_capacity: &usize,
) -> ServiceResult<(Vec<u8>, Vec<u8>, Vec<u8>)> {
    let client = client.clone();
    let get_req = GetObjectRequest {
        bucket: bucket.to_owned(),
        key: path_file.to_string(),
        ..Default::default()
    };

    let res = tokio::spawn(async move { client.get_object(get_req).await })
        .await
        .map_err(|err| {
            debug!("Spawn task failed: {}", err);
            ServiceError::InternalServerError
        })?;

    match res {
        Ok(result) => {
            let stream = result.body.ok_or_else(|| {
                debug!("Empty body for file: {}", path_file);
                ServiceError::InternalServerError
            })?;
            // calculated blake3 for hash
            // let mut blake3_hasher = blake3::Hasher::new_derive_key(DERIVE_KEY);
            let mut blake3_hasher = blake3::Hasher::new();
            // create a Sha256 object
            let mut sha256_hasher = Sha256::new();
            let mut body = stream.into_async_read();
            let mut buffer = bytes::BytesMut::with_capacity(*buffer_capacity);

            let mut header_bytes = Vec::new();
            let mut is_first_chunk = true;

            loop {
                let result = body.read_buf(&mut buffer).await.map_err(|e| {
                    debug!("Failed to read file body: {}", e);
                    ServiceError::InternalServerError
                })?;
                if result == 0 {
                    break;
                }
                if is_first_chunk {
                    let take_len = cmp::min(result, 64);
                    header_bytes = buffer[..take_len].to_vec();
                    is_first_chunk = false;
                }
                blake3_hasher.update(&buffer[..result]);
                sha256_hasher.update(&buffer[..result]);

                buffer.clear();
            }
            let final_hash = blake3_hasher.finalize().as_bytes().to_vec();
            let final_hash_sha256 = sha256_hasher.finalize().to_vec();
            Ok((final_hash, final_hash_sha256, header_bytes))
        }
        Err(err) => {
            debug!("Err get file: {:#?}", err);
            Err(ServiceError::InternalServerError)
        }
    }
}

/// Delete object by path
pub(crate) async fn delete_object_by_path(
    client: &S3Client,
    bucket: &str,
    path_file: &str,
) -> bool {
    let req = rusoto_s3::DeleteObjectRequest {
        bucket: bucket.to_string(),
        key: path_file.to_string(),
        ..Default::default()
    };

    // self.client.as_ref().unwrap().delete_object

    debug!("DeleteObjectRequest: {:#?}", req);

    let client = client.clone();
    let res = tokio::spawn(async move { client.delete_object(req).await }).await;

    // debug!("DeleteObjectRequest: {:#?}", res);
    res.is_ok()
}

/// Updates Content-Type via self-copy with REPLACE
pub(crate) async fn update_s3_metadata(
    client: &S3Client,
    bucket: &str,
    path_file: &str,
    current_type: &str,
    detected_type: &str,
) -> ServiceResult<()> {
    if detected_type != current_type && detected_type != "application/octet-stream" {
        let source_path = format!("{}/{}", bucket, path_file);

        let copy_req = CopyObjectRequest {
            bucket: bucket.to_string(),
            key: path_file.to_string(),
            copy_source: source_path,
            metadata_directive: Some("REPLACE".to_string()),
            content_type: Some(detected_type.to_string()),
            ..Default::default()
        };

        client.copy_object(copy_req).await.map_err(|err| {
            debug!(
                "Failed to update S3 content_type for {}: {:?}",
                path_file, err
            );
            ServiceError::InternalServerError
        })?;
    }

    Ok(())
}
