use std::{io::Write, path::Path};

use serde::{de::DeserializeOwned, Serialize};

use model::{File, GetRequest, ListRequest, ListResponse, ResponseModel};

pub mod model;

pub struct Client {
    client: reqwest::Client,
    host: String,
    api_token: String,
}

impl Client {
    pub fn new(host: String, api_token: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            host,
            api_token,
        }
    }

    async fn post<I: Serialize, T: DeserializeOwned>(
        &self,
        url: &str,
        json: &I,
    ) -> anyhow::Result<T> {
        let response = self
            .client
            .post(format!("{}{}", self.host, url))
            .header("Authorization", &self.api_token)
            .json(json)
            .send()
            .await?;

        if !response.status().is_success() {
            let u = response.url().to_string();
            let status = response.status();
            let body = response
                .text()
                .await
                .unwrap_or_else(|e| format!("parse response body to string error, {}", e));

            return Err(anyhow::Error::msg(format!(
                "http post {u} failed, status: {status}, body: {body}"
            )));
        }

        let r = response.json::<ResponseModel<T>>().await?;
        if r.code != 200 {
            return Err(anyhow::Error::msg(format!(
                "http post failed, code: {}, message: {}",
                r.code, r.message
            )));
        }

        return Ok(r.data.unwrap());
    }

    pub async fn list(&self, path: &str) -> anyhow::Result<Vec<File>> {
        let mut response: ListResponse = self
            .post(
                "/api/fs/list",
                &ListRequest {
                    path: path,
                    page: 1,
                    per_page: 0,
                    refresh: true,
                    password: "",
                },
            )
            .await?;

        for f in response.content.as_mut_slice() {
            f.path = Path::new(path)
                .join(f.name.as_str())
                .to_str()
                .unwrap()
                .to_string();
        }

        Ok(response.content)
    }

    async fn get(&self, url: &str) -> anyhow::Result<File> {
        let file: File = self
            .post(
                "/api/fs/get",
                &GetRequest {
                    path: url,
                    password: "",
                },
            )
            .await?;

        Ok(file)
    }

    pub async fn download(&self, path: &str, save_to: &str) -> anyhow::Result<()> {
        let file = self.get(path).await?;

        let mut res =
            self.client
                .get(file.raw_url.as_str())
                .send()
                .await
                .or(Err(anyhow::Error::msg(format!(
                    "failed to get from '{}'",
                    file.raw_url.as_str()
                ))))?;
        let total_size = res.content_length().unwrap();

        let mut fp = std::fs::File::create(save_to).or(Err(anyhow::Error::msg(format!(
            "Failed to create file '{}'",
            path
        ))))?;
        let mut downloaded: u64 = 0;

        while let Some(chunk) = res.chunk().await? {
            fp.write_all(&chunk).or(Err(anyhow::Error::msg(format!(
                "Error while writing to file"
            ))))?;
            downloaded += chunk.len() as u64;
            tracing::info!("downloading {}, {}/{}", path, downloaded, total_size);
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::Client;

    #[tokio::test]
    async fn test() {
        let host = "http://127.0.0.1:20280";
        let api_token = "alist-df4d4e39-02bb-4554-8696-8ee929a4bf12UKaBfzv03WqX2oPEYhgKujLm8EjbFyibR1pB3EBkq3LX2pzFQEqYqLkWERJnnPJk";

        let client = Client::new(host.to_string(), api_token.to_string());
        let r = client
            .list("/aliyunpan/tv-series/长风渡 (2023)/Season 01")
            .await
            .unwrap();

        println!("{:#?}", r);
    }
}
