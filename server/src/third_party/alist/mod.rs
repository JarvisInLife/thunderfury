pub mod model;

#[cfg(test)]
mod test {
    use super::model::{ListRequest, ListResponse, ResponseModel};

    #[tokio::test]
    async fn test() {
        let client = reqwest::Client::new();
        let response = client
            .post("http://127.0.0.1:20280/api/fs/list")
            .json(&ListRequest {
                path: "/aliyunpan/tv-series/长风渡 (2023)/Season 01".to_string(),
                page: 1,
                per_page: 0,
                refresh: true,
                ..Default::default()
            })
            .send()
            .await
            .unwrap();

        let r = response
            .json::<ResponseModel<ListResponse>>()
            .await
            .unwrap();

        println!("{:#?}", r);
    }
}
