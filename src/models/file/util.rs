use super::model::{FileMetadata, SlimFile};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::extension::model::InsertableExtension;
use crate::models::extension::service::register::create_extension;
use crate::schema::file_ref::dsl as file_ref;
use crate::storage::s3_client::get_object_body;
use diesel::prelude::*;
use rusoto_s3::S3Client;
use std::path::Path;
use uuid::Uuid;

/// Row `unknown` in `extension_ref` and `program_ref`
const UNKNOWN_ID: i32 = 1;
/// Guard against garbage "extensions" (real ones are <= 10 chars)
const MAX_EXT_LEN: usize = 20;

/// Calculate hash and define the file extension
pub(crate) async fn get_metadata(
    client: &S3Client,
    bucket: &str,
    buffer_capacity: &usize,
    slim_file: &SlimFile,
    conn: &PgConnection,
) -> ServiceResult<FileMetadata> {
    let (blake3_hash, sha256_hash, header_bytes) =
        get_object_body(client, bucket, &slim_file.path_file, buffer_capacity).await?;

    // get id for extension
    let id_ext = find_id_ext(&slim_file.filename, conn)?;

    // detect MIME type from magic bytes or file extension
    let content_type = detect_mime_type(&slim_file.filename, &header_bytes);

    Ok(FileMetadata {
        blake3_hash,
        sha256_hash,
        content_type,
        id_ext,
    })
}

/// Get extension id on table for file extension
/// if not found, add new
fn find_id_ext(filename: &str, conn: &PgConnection) -> ServiceResult<i32> {
    use crate::schema::extension_ref::dsl as extension_ref;
    // debug!("Filename_str {:?}", filename);
    let ext_str = Path::new(filename)
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .unwrap_or_default();
    // debug!("Ext_str {:?}", ext_str);
    if ext_str.is_empty() {
        return Ok(UNKNOWN_ID);
    }
    // find extension id, register missing extensions as generic (unknown program)
    let get_ext = extension_ref::extension_ref
        .filter(extension_ref::extension.eq(&ext_str))
        .select(extension_ref::id)
        .order(extension_ref::id.asc())
        .first::<i32>(conn)
        .optional()
        .map_err(|err| {
            debug!("Failed get extension_ref: {:?}", err);
            ServiceError::InternalServerError
        })?;

    match get_ext {
        Some(id) => Ok(id),
        // not found ext, add new
        None => {
            // chech valid
            if ext_str.len() < MAX_EXT_LEN {
                let new_extension_data = InsertableExtension {
                    extension: ext_str,
                    program_id: UNKNOWN_ID,
                };
                create_extension(&new_extension_data, conn)
            } else {
                Err(ServiceError::InternalServerError)
            }
        }
    }
}

/// Sets empty hash for no parsing file in future
pub(crate) fn set_skip_file(file_uuid: &Uuid, conn: &PgConnection) -> ServiceResult<bool> {
    let zero_hash: Vec<u8> = vec![0; 64];

    diesel::update(file_ref::file_ref.filter(file_ref::uuid.eq(file_uuid)))
        .set((
            file_ref::hash.eq(zero_hash.clone()),
            file_ref::sha256_hash.eq(zero_hash),
        ))
        .returning(file_ref::is_delete)
        .get_result(conn)
        .map_err(|err| {
            debug!("Failed set zero hash file: {:?}", err);
            ServiceError::InternalServerError
        })
}

pub(crate) fn detect_mime_type(filename: &str, header_bytes: &[u8]) -> String {
    let mime_type = infer::get(header_bytes)
        .map(|kind| kind.mime_type().to_string())
        .unwrap_or_else(|| "application/octet-stream".to_string());

    if mime_type != "application/octet-stream" {
        return mime_type;
    }

    let ext = std::path::Path::new(filename)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "step" | "stp" => "application/step".to_string(),
        "iges" | "igs" => "application/iges".to_string(),
        "stl" => "model/stl".to_string(),
        _ => "application/octet-stream".to_string(),
    }
}
