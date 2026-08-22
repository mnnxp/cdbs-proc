use crate::cli_args::Opt;
use crate::errors::{ServiceError, ServiceResult};
use crate::schema::*;
use chrono::{NaiveDateTime, Utc};
use structopt::StructOpt;
use uuid::Uuid;

/// Saving an active link to the file for uses the cache browser
#[derive(Insertable, Debug)]
#[table_name = "presigned_url_ref"]
pub(crate) struct InsertablePresignedUrl {
    pub(crate) file_uuid: Uuid,
    pub(crate) presigned_url: String,
    pub(crate) expiration_at: NaiveDateTime,
}

#[derive(Clone, Debug)]
pub(crate) struct StorageAccess {
    application_key_id: String,
    application_key: String,
    pub(crate) bucket: String,
    pub(crate) region: String,
    pub(crate) endpoint: String,
}

impl StorageAccess {
    /// Gets data to access S3 from environment for generate presign-urls
    pub(crate) fn from_env() -> ServiceResult<StorageAccess> {
        let opt = Opt::from_args();

        // checking expiration date for key
        if opt.s3_access_expiration_at < Utc::now().naive_utc() {
            return Err(ServiceError::BadRequest("S3 key expired".into()));
        }

        Ok(StorageAccess {
            application_key_id: opt.s3_application_key_id,
            application_key: opt.s3_application_key,
            bucket: opt.s3_bucket,
            region: opt.s3_region,
            endpoint: opt.s3_endpoint,
        })
    }
}

impl From<&StorageAccess> for super::s3::Aws {
    fn from(data: &StorageAccess) -> super::s3::Aws {
        // todo!(add encryptind)
        let application_key_id = &data.application_key_id;
        let application_key = &data.application_key;

        super::s3::Aws::new(
            application_key_id,
            application_key,
            &data.region,
            &data.endpoint,
        )
    }
}
