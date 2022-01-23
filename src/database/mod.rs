pub(crate) mod pool;

use crate::errors::ServiceError;
use diesel::r2d2::PoolError;

type ConnectionManager = diesel::r2d2::ConnectionManager<diesel::PgConnection>;
pub(crate) type Pool = diesel::r2d2::Pool<ConnectionManager>;
pub(crate) type PooledConnection = diesel::r2d2::PooledConnection<ConnectionManager>;
pub(crate) type PgPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

pub(crate) fn db_connection(pool: &Pool) -> Result<PooledConnection, ServiceError> {
    pool.get().map_err(|_| ServiceError::UnableToConnectToDb)
}
