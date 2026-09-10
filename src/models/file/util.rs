use super::model::{FileMetadata, SlimFile};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::extension::model::InsertableExtension;
use crate::models::extension::service::register::create_extension;
use crate::schema::file_ref::dsl as file_ref;
use crate::storage::s3_client::get_object_body;
use diesel::prelude::*;
use regex::Regex;
use rusoto_s3::S3Client;
use uuid::Uuid;

/// Calculate hash and define the file extension
pub(crate) async fn get_metadata(
    client: &S3Client,
    bucket: &str,
    buffer_capacity: &usize,
    slim_file: &SlimFile,
    conn: &PgConnection,
) -> ServiceResult<FileMetadata> {
    let (blake3_hash, sha256_hash) =
        get_object_body(client, bucket, &slim_file.path_file, buffer_capacity).await?;

    // get id for extension
    let id_ext = find_id_ext(&slim_file.filename, conn)?;

    // get metadata for filesize
    // let size = body.len() as u64;

    Ok(FileMetadata {
        blake3_hash,
        sha256_hash,
        id_ext,
    })
}

/// Get extension id on table for file extension
/// if not found, add new
fn find_id_ext(filename: &str, conn: &PgConnection) -> ServiceResult<i32> {
    use crate::schema::extension_ref::dsl as extension_ref;
    // debug!("Filename_str {:?}", filename);
    let ext_str = Regex::new(r"\.\w+$")
        .unwrap()
        .find(filename)
        .map(|m| m.as_str())
        .unwrap_or_default();
    // debug!("Ext_str {:?}", ext_str);
    if ext_str.is_empty() {
        return Ok(1);
    }
    // find id extension or set not found id = 1
    let get_ext = extension_ref::extension_ref
        .filter(extension_ref::extension.eq(ext_str))
        .select(extension_ref::id)
        .limit(1)
        .load::<i32>(conn)
        .map_err(|err| {
            debug!("Failed get extension_ref: {:?}", err);
            ServiceError::InternalServerError
        })?;

    match get_ext.first() {
        Some(x) => Ok(*x),
        // not found ext, add new
        None => {
            // chech valid
            if ext_str.len() < 10 {
                let new_extension_data = InsertableExtension {
                    extension: ext_str.to_string(),
                    program_id: 1, // unknown
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
