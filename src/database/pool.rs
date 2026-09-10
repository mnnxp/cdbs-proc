use super::{ConnectionManager, Pool, PoolError};
use std::time::Duration;

fn init_pool(database_url: &str) -> Result<Pool, PoolError> {
    let manager = ConnectionManager::new(database_url);
    Pool::builder()
        .max_size(5)
        .min_idle(Some(1))
        .connection_timeout(Duration::from_secs(10))
        // .idle_timeout(Some(Duration::from_secs(600)))
        .build(manager)
}

pub(crate) fn establish_connection(opt: crate::cli_args::Opt) -> Pool {
    let database_url = format!(
        "postgres://{user}:{password}@{host}/{database}",
        user = opt.postgres_user,
        password = opt.postgres_password,
        host = opt.postgres_host,
        database = opt.postgres_db
    );
    init_pool(&database_url).expect("Failed to create pool")
}
