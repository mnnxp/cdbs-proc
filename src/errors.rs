use diesel::result::Error as DBError;
use std::convert::From;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HostingError {
    #[error("Error whith access to S3")]
    BadRequest(#[from] reqwest::Error),
}

#[derive(Debug, Error, Serialize, Clone)]
pub enum ServiceError {
    #[error("Internal Server Error")]
    InternalServerError,

    #[error("BadRequest: {0}")]
    BadRequest(String),

    #[error("Unable to connect to DB")]
    UnableToConnectToDb,
}

// return early in our handlers if UUID provided by the user is not valid
impl From<uuid::Error> for ServiceError {
    fn from(_: uuid::Error) -> ServiceError {
        ServiceError::BadRequest("Invalid UUID".into())
    }
}

// return tokio error of spawn
impl From<tokio::task::JoinError> for ServiceError {
    fn from(err: tokio::task::JoinError) -> ServiceError {
        ServiceError::BadRequest(format!("{:?}", err))
    }
}

impl From<DBError> for ServiceError {
    fn from(error: DBError) -> ServiceError {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match error {
            DBError::DatabaseError(_kind, info) => {
                let message = info.details().unwrap_or_else(|| info.message()).to_string();
                ServiceError::BadRequest(message)
            }
            _ => ServiceError::InternalServerError,
        }
    }
}

pub type ServiceResult<V> = std::result::Result<V, crate::errors::ServiceError>;
