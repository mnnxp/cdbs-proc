use structopt::StructOpt;
use chrono::NaiveDateTime;

/// GraphQl API, Diesel PostgreSQL, session authentication and JWT boilerplate server
#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "cdbs-parser")]
pub(crate) struct Opt {
    /// Limiting the selection of objects from the database in one cycle
    #[structopt(long, env = "LIMIT_PART", default_value = "10")]
    pub(crate) limit_part: i64,

    /// Set capacity for BytesMut, this does not specify the length, but only the capacity
    #[structopt(long, env = "BUFFER_CAPACITY", default_value = "64")]
    pub(crate) buffer_capacity: usize,

    /// Sleeping time in milliseconds
    #[structopt(long, env = "SLEEPING_TIME", default_value = "43200000")]
    pub(crate) sleeping_time: u64,

    /// Database host
    #[structopt(long, env = "POSTGRES_HOST")]
    pub(crate) postgres_host: String,

    /// Database user
    #[structopt(long, env = "POSTGRES_USER")]
    pub(crate) postgres_user: String,

    /// Database password
    #[structopt(long, env = "POSTGRES_PASSWORD")]
    pub(crate) postgres_password: String,

    /// Database name database
    #[structopt(long, env = "POSTGRES_DB")]
    pub(crate) postgres_db: String,

    /// Application key id for S3
    #[structopt(long, env = "S3_APPLICATION_KEY_ID")]
    pub(crate) s3_application_key_id: String,

    /// Application key for S3
    #[structopt(long, env = "S3_APPLICATION_KEY")]
    pub(crate) s3_application_key: String,

    /// Key expiration date for S3
    #[structopt(long, env = "S3_ACCESS_EXPIRATION_AT")]
    pub(crate) s3_access_expiration_at: NaiveDateTime,

    /// Bucket name from S3
    #[structopt(long, env = "S3_BUCKET")]
    pub(crate) s3_bucket: String,

    /// Server location S3
    #[structopt(long, env = "S3_REGION")]
    pub(crate) s3_region: String,

    /// Url to endpoint S3
    #[structopt(long, env = "S3_ENDPOINT")]
    pub(crate) s3_endpoint: String,
}
