use rusoto_signature::{credential::AwsCredentials, Region};

#[derive(Clone)]
pub(crate) struct Aws {
    credentials: AwsCredentials,
    region: Region,
}

impl Aws {
    /// Create new Aws access
    pub(crate) fn new(
        access_key_id: &str,
        secret_access_key: &str,
        region: &str,
        endpoint: &str,
    ) -> Aws {
        let credentials = AwsCredentials::new(access_key_id, secret_access_key, None, None);

        // debug!("Credentials: {:#?}", credentials);

        let region = Region::Custom {
            name: region.to_string(),
            endpoint: endpoint.to_string(),
        };

        // debug!("Region: {:#?}", region);

        Aws {
            credentials,
            region,
        }
    }

    /// Return cloned a AwsCredentials (used for create S3Client)
    pub(super) fn clone_credentials(&self) -> AwsCredentials {
        self.credentials.clone()
    }

    /// Return cloned a Region (used for create S3Client)
    pub(super) fn clone_region(&self) -> Region {
        self.region.clone()
    }
}
