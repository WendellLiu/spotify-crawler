use elasticsearch::{http::transport::Transport, Elasticsearch, Error};

use crate::config::SystemConfig;

pub struct EsClient {
    pub client: Elasticsearch,
}

impl EsClient {
    pub async fn new() -> Result<Self, Error> {
        let system_config = SystemConfig::global();
        let endpoint = system_config.elasticsearch.endpoint.clone();
        let transport = Transport::single_node(&endpoint)?;
        let client = Elasticsearch::new(transport);
        Ok(EsClient { client: client })
    }
}
