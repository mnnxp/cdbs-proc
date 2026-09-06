use crate::database::PgPool;
use crate::errors::{ServiceError, ServiceResult};
use crate::models::file::{model::SlimFile, util::get_metadata};
use crate::schema::file_ref::dsl as file_ref;
use chrono::Utc;
use diesel::prelude::*;

/// Update file metadata by SlimFile
pub(crate) async fn update_metadata(
    client: &rusoto_s3::S3Client,
    bucket: &str,
    buffer_capacity: &usize,
    slim_file: &SlimFile,
    pool: &PgPool,
) -> ServiceResult<usize> {
    let conn = pool.get().map_err(|_| ServiceError::UnableToConnectToDb)?;

    let file_metadata = get_metadata(client, bucket, buffer_capacity, slim_file, pool).await?;

    let (parent_file_uuid, revision) =
        diesel::update(file_ref::file_ref.filter(file_ref::uuid.eq(&slim_file.uuid)))
            .set((
                file_ref::hash.eq(&file_metadata.blake3_hash),
                file_ref::sha256_hash.eq(&file_metadata.sha256_hash),
                file_ref::id_ext.eq(&file_metadata.id_ext),
                file_ref::is_checked.eq(true),
            ))
            .returning((file_ref::parent_file_uuid, file_ref::revision))
            .get_result::<(uuid::Uuid, i32)>(&conn)
            .map_err(|err| {
                debug!("Failed set metadata: {:?}", err);
                ServiceError::BadRequest("Failed set metadata".to_string())
            })?;

    // hide the old version of a file if the file has a link to another file
    if parent_file_uuid != slim_file.uuid && revision > 1 {
        return diesel::update(file_ref::file_ref.filter(file_ref::uuid.eq(&parent_file_uuid)))
            .set((
                file_ref::is_hidden.eq(true),
                file_ref::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(&conn)
            .map_err(|err| {
                debug!("Failed set metadata: {:?}", err);
                ServiceError::BadRequest("Failed set metadata".to_string())
            });
    }

    Ok(1)
}
