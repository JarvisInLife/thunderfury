pub mod model;

pub use model::TmdbError;
use model::TvDetail;
use reqwest::{IntoUrl, StatusCode};
use serde::de::DeserializeOwned;
use std::fmt::Display;

const TMDB_HOST: &str = "https://api.themoviedb.org/3";

macro_rules! build_url {
    ($($arg:tt)*) => {{
        let res = format!("{}{}", TMDB_HOST, format!($($arg)*));
        res
    }}
}

pub struct Client {
    client: reqwest::Client,
    api_key: String,
}

impl Client {
    pub fn new(api_key: String) -> Self {
        Client {
            client: reqwest::Client::new(),
            api_key,
        }
    }

    async fn get<U: IntoUrl + Display, T: DeserializeOwned>(&self, url: U) -> Result<T, TmdbError> {
        let response = self
            .client
            .get(url)
            .query(&[("language", "zh-CN"), ("api_key", self.api_key.as_str())])
            .send()
            .await?;

        if response.status().is_success() {
            return Ok(response.json().await?);
        }

        let u = response.url().to_string();
        let status = response.status();
        let body = response
            .text()
            .await
            .unwrap_or_else(|e| format!("parse response body to string error, {}", e));

        match status {
            StatusCode::UNAUTHORIZED => Err(TmdbError::Unauthorized(body)),
            StatusCode::NOT_FOUND => Err(TmdbError::NotFound),
            _ => Err(TmdbError::Other(format!(
                "http get {u} failed, status: {status}, body: {body}"
            ))),
        }
    }

    pub async fn get_tv_detail(&self, id: i32) -> Result<TvDetail, TmdbError> {
        Ok(self.get(build_url!("/tv/{}", id)).await?)
    }
}

#[cfg(test)]
mod test {
    use super::Client;

    #[tokio::test]
    async fn it_works() {
        let res = Client::new("".to_owned()).get_tv_detail(1).await;
        match res {
            Ok(_) => {}
            Err(e) => {
                println!("{}", e);
                assert!(e.to_string().contains("401 Unauthorized"));
            }
        }
    }
}
