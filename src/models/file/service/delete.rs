use crate::errors::{ServiceError, ServiceResult};
use crate::database::PgPool;
use crate::models::file::model::SlimFile;
use crate::storage::s3_client::delete_object_by_path;
use crate::schema::file_ref::dsl as file_ref;
use diesel::prelude::*;
use uuid::Uuid;

/// Delete file in storage and row in database
pub(crate) async fn delete_file(
    client: &rusoto_s3::S3Client,
    bucket: &str,
    slim_file: &SlimFile,
    pool: &PgPool,
) -> ServiceResult<bool> {
    let conn = pool.get().unwrap();

    let res = delete_object_by_path(client, bucket, &slim_file.path_file).await;

    // delete row about file in database
    let _path_file = delete_file_by_uuid(&slim_file.uuid, &conn)?;

    // delete file in storage
    Ok(res)
}

/// Delete row in database
fn delete_file_by_uuid(
    file_uuid: &Uuid,
    conn: &PgConnection,
) -> ServiceResult<String> {
    diesel::delete(file_ref::file_ref)
        .filter(file_ref::uuid.eq(file_uuid))
        .returning(file_ref::path_file)
        .get_result::<String>(conn)
        .map_err(|err| {
            debug!("Failded delete file record in database : {:?}", err);
            ServiceError::InternalServerError
        })
}
