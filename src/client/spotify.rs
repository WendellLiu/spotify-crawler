use reqwest::header::AUTHORIZATION;
use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};

use crate::config::SystemConfig;

pub struct SpotifyClient {
    pub token: String,
    pub endpoint: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewReleaseResponse {}

impl SpotifyClient {
    pub fn new(token: &String) -> SpotifyClient {
        let system_config = SystemConfig::global();

        SpotifyClient {
            token: token.clone(),
            endpoint: system_config.spotify.endpoint.clone(),
        }
    }

    fn get(&self, namespace: String) -> RequestBuilder {
        let client = Client::new();
        let token = format!("Bearer {}", self.token);
        let url = format!("{}{}", self.endpoint, namespace);
        client.get(&url).header(AUTHORIZATION, token)
    }

    fn post<Body: Serialize>(&self, namespace: String, body: Body) -> RequestBuilder {
        let client = Client::new();
        let token = format!("Bearer {}", self.token);
        let url = format!("{}{}", self.endpoint, namespace);
        client.post(&url).header(AUTHORIZATION, token).json(&body)
    }

    pub async fn get_new_releases(&self) -> Result<NewReleaseResponse, reqwest::Error> {
        self.get(String::from("/v1/browse/new-releases"))
            .send()
            .await?
            .json()
            .await
    }
}
