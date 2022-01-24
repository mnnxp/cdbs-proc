use crate::errors::{ServiceResult, ServiceError};
use rusoto_core::request::HttpClient;
use rusoto_s3::{GetObjectRequest, S3, S3Client};
use rusoto_signature::credential::StaticProvider;
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
    buffer_capacity: &usize
) -> ServiceResult<Vec<u8>> {
    let client = client.clone();
    let get_req = GetObjectRequest {
        bucket: bucket.to_owned(),
        key: path_file.to_string(),
        ..Default::default()
    };

    let res = tokio::spawn(async move {
        client.get_object(get_req).await
    }).await.unwrap();

    match res {
        Ok(result) => {
            let stream = result.body.unwrap();
            // calculated blake3 for hash
            // let mut hasher = blake3::Hasher::new_derive_key(DERIVE_KEY);
            let mut hasher = blake3::Hasher::new();
            let mut body = stream.into_async_read();
            let mut buffer = bytes::BytesMut::with_capacity(*buffer_capacity);
            loop {
                let result = body.read_buf(&mut buffer).await.unwrap();
                if result == 0 { break }
                hasher.update(&buffer[..result]);
                // We never read uninitialized data from the buffer, so this is OK. `read_buf`
                // will set the bytes and we only pass read bytes in the slice to the hasher.
                unsafe {
                    buffer.set_len(0);
                };
            }
            let final_hash = hasher.finalize().as_bytes().to_vec();
            Ok(final_hash)
        },
        Err(err) => {
            debug!("Err get file: {:#?}", err);
            Err(ServiceError::InternalServerError)
        },
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
    let res = tokio::spawn(async move {
        client.delete_object(req).await
    }).await;

    // debug!("DeleteObjectRequest: {:#?}", res);
    res.is_ok()
}
