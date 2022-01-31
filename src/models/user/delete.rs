use crate::errors::ServiceError;
use crate::schema::user_ref::dsl as user_ref;
use diesel::prelude::*;

/// Clear removed users data of database
pub(crate) fn clear_removed_users(conn: &PgConnection) {
    let clear_users = diesel::delete(user_ref::user_ref)
        .filter(user_ref::is_enabled.eq(false)
        .and(user_ref::is_delete.eq(true)))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed clear removes users data: {:?}", err);
            ServiceError::InternalServerError
        });

    debug!("clear_users {:?}", clear_users);
}
