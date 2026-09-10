use crate::errors::{ServiceError, ServiceResult};
use crate::schema::user_api_key_ref::dsl::*;
use chrono::Utc;
use diesel::prelude::*;

/// Deactivates API keys that have passed their expiration date
pub(crate) fn deactivate_expired_keys(conn: &mut PgConnection) -> ServiceResult<usize> {
    let now = Utc::now();

    diesel::update(user_api_key_ref)
        .filter(expires_at.lt(now))
        .filter(is_active.eq(true))
        .set(is_active.eq(false))
        .execute(conn)
        .map_err(|e| {
            debug!("Failed to deactivate expired API keys: {}", e);
            ServiceError::InternalServerError
        })
}
