use crate::errors::{ServiceError, ServiceResult};
use crate::database::PgPool;
use crate::models::file::{
    model::SlimFile,
    util::get_metadata,
};
use crate::schema::file_ref::dsl as file_ref;
use diesel::prelude::*;

/// Update file metadata by SlimFile
pub(crate) async fn update_metadata(
    client: &rusoto_s3::S3Client,
    bucket: &str,
    buffer_capacity: &usize,
    slim_file: &SlimFile,
    pool: &PgPool,
) -> ServiceResult<usize> {
    let conn = pool.get().unwrap();

    let file_metadata = get_metadata(
        client,
        bucket,
        buffer_capacity,
        slim_file,
        pool
    ).await?;

    diesel::update(file_ref::file_ref.filter(file_ref::uuid.eq(&slim_file.uuid)))
        .set((
            file_ref::hash.eq(&file_metadata.hash),
            file_ref::id_ext.eq(&file_metadata.id_ext),
            file_ref::is_checked.eq(true),
            file_ref::updated_at.eq(chrono::Local::now().naive_local())
        ))
        .execute(&conn)
        .map_err(|err| {
            debug!("Failed set metadata: {:?}", err);
            ServiceError::BadRequest("Failed set metadata".to_string())
        })
}
