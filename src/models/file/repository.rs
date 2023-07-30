use super::model::SlimFile;
use crate::errors::{ServiceError, ServiceResult};
use crate::schema::file_ref::dsl as file_ref;
use diesel::prelude::*;

impl SlimFile {
    /// Get row for parsing hash
    /// filter: no hash, not empty, not checked, not hidden, not deleted.
    pub(crate) fn get_files_for_check(
        limit: &i64,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<SlimFile>> {
        let empty_hash: Vec<u8> = Vec::new();

        file_ref::file_ref
            .select((
                file_ref::uuid,
                file_ref::filename,
                // file_ref::filesize,
                file_ref::path_file,
            ))
            .filter(file_ref::hash.eq(&empty_hash)
                .and(file_ref::is_checked.eq(false))
                .and(file_ref::is_hidden.eq(false))
                .and(file_ref::is_delete.eq(false))
                .and(file_ref::filesize.gt(0)))
            .limit(*limit)
            .load::<SlimFile>(conn)
            .map_err(|err| {
                debug!("Failed get file: {:?}", err);
                ServiceError::InternalServerError
            })
    }

    /// Get row with flag is_delete is true for delete with limit
    pub(crate) fn get_for_delete(
        limit: &i64,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<SlimFile>> {
        file_ref::file_ref
            .select((
                file_ref::uuid,
                file_ref::filename,
                // file_ref::filesize,
                file_ref::path_file,
            ))
            .filter(file_ref::is_checked.eq(false)
                .and(file_ref::is_delete.eq(true)))
            .limit(*limit)
            .load::<SlimFile>(conn)
            .map_err(|err| {
                debug!("Failed get file: {:?}", err);
                ServiceError::InternalServerError
            })
    }
}
