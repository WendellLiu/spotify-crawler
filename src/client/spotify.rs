use base64::encode;
use reqwest::header::AUTHORIZATION;
use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::config::SystemConfig;

#[derive(Debug)]
pub struct SpotifyClient {
    pub token: String,
    pub endpoint: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum TokenResponse {
    Token {
        access_token: String,
        token_type: String,
        expires_in: u16,
    },
    AuthenticationError {
        error: String,
        error_description: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewReleaseResponse {}

impl SpotifyClient {
    pub async fn new() -> Result<SpotifyClient, reqwest::Error> {
        let system_config = SystemConfig::global();
        let endpoint = system_config.spotify.auth_endpoint.clone();
        let client_id = system_config.spotify.client_id.clone();
        let client_secret = system_config.spotify.client_secret.clone();

        let url = format!("{}{}", endpoint, "/token");
        let credential = encode(format!("{}:{}", client_id, client_secret));
        let authorization_content = format!("Basic {}", credential);

        let mut params = HashMap::new();
        params.insert("grant_type", "client_credentials");

        let client = Client::new();

        let response = client
            .post(&url)
            .header(AUTHORIZATION, authorization_content)
            .form(&params)
            .send()
            .await?;

        let resp: TokenResponse = response.json().await?;

        let token = match resp {
            TokenResponse::Token { access_token, .. } => access_token,
            TokenResponse::AuthenticationError { error, .. } => panic!("{:?}", error),
        };

        Ok(SpotifyClient {
            token: token,
            endpoint: endpoint,
        })
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
