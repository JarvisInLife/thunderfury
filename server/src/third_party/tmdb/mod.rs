pub mod model;

use std::fmt::Display;

use model::TvDetail;
use reqwest::IntoUrl;
use serde::de::DeserializeOwned;

const TMDB_HOST: &str = "https://api.themoviedb.org/3";

macro_rules! build_url {
    ($($arg:tt)*) => {{
        let res = format!("{}{}", TMDB_HOST, format!($($arg)*));
        res
    }}
}

pub struct Client {
    client: reqwest::Client,
    token: String,
}

impl Client {
    pub fn new(token: String) -> Self {
        Client {
            client: reqwest::Client::new(),
            token,
        }
    }

    async fn get<U: IntoUrl + Display, T: DeserializeOwned>(&self, url: U) -> anyhow::Result<T> {
        let response = self.client.get(url).bearer_auth(&self.token).send().await?;
        if !response.status().is_success() {
            // http request failed
            let u = response.url().to_string();
            let status = response.status();
            let body = response
                .text()
                .await
                .unwrap_or_else(|e| format!("parse response body to string error, {}", e));

            return Err(anyhow::anyhow!(
                "http get {u} failed, status: {status}, body: {body}",
            ));
        }

        Ok(response.json().await?)
    }

    pub async fn get_tv_detail(&self, id: i32) -> anyhow::Result<TvDetail> {
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
