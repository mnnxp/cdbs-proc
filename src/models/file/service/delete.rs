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

    // delete file in storage
    let file_removed = delete_object_by_path(client, bucket, &slim_file.path_file).await;
    if file_removed {
        // preparing child files before removed a parent file
        let _amount = make_child_independent(&slim_file.uuid, &conn)?;
        // set flags in database about the file is removed
        let _path_file = set_delete_file_by_uuid(&slim_file.uuid, &conn)?;
    }
    Ok(file_removed)
}

/// Set row as delete in database
fn set_delete_file_by_uuid(
    file_uuid: &Uuid,
    conn: &PgConnection,
) -> ServiceResult<String> {
    diesel::update(file_ref::file_ref.filter(file_ref::uuid.eq(file_uuid)))
        .set((
            file_ref::is_checked.eq(true),
            file_ref::is_hidden.eq(true),
            file_ref::is_delete.eq(true),
        ))
        .returning(file_ref::path_file)
        .get_result::<String>(conn)
        .map_err(|err| {
            debug!("Failed set flags for a removed file record in database: {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Replaces parent reference with self
fn make_child_independent(
    file_uuid: &Uuid,
    conn: &PgConnection,
) -> ServiceResult<usize> {
    let child_files = file_ref::file_ref
        .select(file_ref::uuid)
        .filter(file_ref::parent_file_uuid.eq(file_uuid))
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failded select child files record in database: {:?}", err);
            ServiceError::InternalServerError
        })?;
    // debug!("Found dependents in {:?} row(s)", child_files.len());
    // debug!("Found dependents in {:#?} row(s)", child_files);

    let mut counter_child = 0;
    for cf in child_files {
        counter_child += diesel::update(file_ref::file_ref.filter(file_ref::uuid.eq(cf)))
            .set(file_ref::parent_file_uuid.eq(cf))
            .execute(conn)
            .map_err(|err| {
                debug!("Failded update child file record in database: {:?}", err);
                ServiceError::InternalServerError
            })?;
    }
    debug!("Removed dependents in {:?} row(s)", counter_child);
    Ok(counter_child)
}