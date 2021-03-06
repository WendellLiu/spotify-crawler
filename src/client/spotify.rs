use base64::encode;
use reqwest::header::AUTHORIZATION;
use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageObject {
    pub height: u32,
    pub width: u32,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExternalUrlsObject {
    pub spotify: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArtistObject {
    pub id: String,
    pub name: String,
    pub external_urls: ExternalUrlsObject,
    #[serde(default)]
    pub images: Vec<ImageObject>,
    #[serde(default)]
    pub genres: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AlbumItem {
    pub id: String,
    pub name: String,
    pub external_urls: ExternalUrlsObject,
    #[serde(default)]
    pub genres: Vec<String>,
    #[serde(default)]
    pub images: Vec<ImageObject>,
    #[serde(default)]
    pub label: String,
    pub release_date: String,
    pub release_date_precision: String,
    pub total_tracks: u32,
    pub artists: Vec<ArtistObject>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Albums {
    pub items: Vec<AlbumItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewReleaseResponse {
    pub albums: Albums,
}

#[derive(Debug, Serialize, Deserialize, EnumString, Clone)]
pub enum Country {
    SE,
    TW,
    US,
    GB,
    JP,
    KR,
    CN,
    TH,
    DE,
    FR,
    MY,
    ZA,
    DK,
    EG,
    CA,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenreSeedsResponse {
    pub genres: Vec<String>,
}

impl SpotifyClient {
    pub async fn new() -> Result<SpotifyClient, reqwest::Error> {
        let system_config = SystemConfig::global();
        let auth_endpoint = system_config.spotify.auth_endpoint.clone();
        let endpoint = system_config.spotify.endpoint.clone();
        let client_id = system_config.spotify.client_id.clone();
        let client_secret = system_config.spotify.client_secret.clone();

        let url = format!("{}{}", auth_endpoint, "/token");
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

    fn get<T: Serialize + ?Sized>(
        &self,
        namespace: String,
        query: Option<std::collections::HashMap<&str, Box<T>>>,
    ) -> RequestBuilder {
        let client = Client::new();
        let token = format!("Bearer {}", self.token);
        let url = format!("{}{}", self.endpoint, namespace);

        let q = match query {
            None => return client.get(&url).header(AUTHORIZATION, token),
            Some(qu) => qu,
        };

        let mut query_map = HashMap::new();
        for (k, v) in q.iter() {
            query_map.insert(*k, v);
        }

        client
            .get(&url)
            .header(AUTHORIZATION, token)
            .query(&query_map)
    }

    fn post<Body: Serialize>(&self, namespace: String, body: Body) -> RequestBuilder {
        let client = Client::new();
        let token = format!("Bearer {}", self.token);
        let url = format!("{}{}", self.endpoint, namespace);
        client.post(&url).header(AUTHORIZATION, token).json(&body)
    }

    pub async fn get_new_releases(
        &self,
        country: Option<Country>,
    ) -> Result<NewReleaseResponse, reqwest::Error> {
        let query = match country {
            Some(coun) => {
                let mut q = HashMap::new();
                q.insert("country", Box::new(coun));
                Some(q)
            }
            None => None,
        };
        self.get(String::from("/v1/browse/new-releases"), query)
            .send()
            .await?
            .json()
            .await
    }
    pub async fn get_genre_seeds(&self) -> Result<GenreSeedsResponse, reqwest::Error> {
        self.get::<()>(
            String::from("/v1/recommendations/available-genre-seeds"),
            None,
        )
        .send()
        .await?
        .json()
        .await
    }
}
