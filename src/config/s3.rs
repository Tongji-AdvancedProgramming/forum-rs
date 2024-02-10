use std::error::Error;

use minio::s3::{client, creds::StaticProvider};
use serde::Deserialize;

#[derive(Default, Debug, Clone, Deserialize, Eq, PartialEq)]
pub struct S3Config {
    pub endpoint: String,
    pub bucket: String,
    pub access_key: String,
    pub secret_key: String,
    pub base_url: String,
    pub prefix: S3PrefixConfig,
}

#[derive(Default, Debug, Clone, Deserialize, Eq, PartialEq)]
pub struct S3PrefixConfig {
    pub upload: String,
    pub avatar: String,
    pub card_bg: String,
    pub homework_upload: String,
}

pub struct S3Conn {
    pub client: client::Client,
    pub config: S3Config,
}

impl S3Conn {
    pub fn init() -> Result<Self, Box<dyn Error + Send + Sync>> {
        let config = crate::config::get_config();
        let guard = config.read().unwrap();

        let credential_provider =
            StaticProvider::new(&guard.s3.access_key, &guard.s3.secret_key, None);

        let client = client::ClientBuilder::new(guard.s3.endpoint.parse()?)
            .provider(Some(Box::new(credential_provider)))
            .build()?;

        Ok(Self {
            client,
            config: guard.s3.clone(),
        })
    }
}
