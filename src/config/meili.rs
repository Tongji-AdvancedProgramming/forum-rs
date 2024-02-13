use meilisearch_sdk::Client;
use serde::Deserialize;

#[derive(Default, Debug, Clone, Deserialize, Eq, PartialEq)]
pub struct MeiliSearchConfig {
    pub host: String,
    pub key: String,
    pub index: MeiliIndexConfig,
}

#[derive(Default, Debug, Clone, Deserialize, Eq, PartialEq)]
pub struct MeiliIndexConfig {
    pub post: String,
}

pub struct Meili {
    client: Client,
}

impl Meili {
    pub fn init() -> Self {
        let config = crate::config::get_config();
        let guard = config.read().unwrap();

        let client = Client::new(&guard.meili.host, Some(&guard.meili.key));
        Self { client }
    }

    pub fn get_client(&self) -> &Client {
        &self.client
    }
}
