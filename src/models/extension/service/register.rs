use crate::errors::{ServiceError, ServiceResult};
use crate::models::extension::model::InsertableExtension;
use crate::schema::extension_ref::dsl as extension_ref;
use diesel::prelude::*;
// use uuid::Uuid;

pub(crate) fn create_extension(
    new_extension_data: &InsertableExtension,
    conn: &PgConnection
) -> ServiceResult<i32> {
    diesel::insert_into(extension_ref::extension_ref)
        .values(new_extension_data)
        .returning(extension_ref::id)
        .get_result::<i32>(conn)
        .map_err(|err| {
            debug!("Failed insert extension: {:?}", err);
            ServiceError::InternalServerError
        })
}
