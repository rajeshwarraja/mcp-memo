// Project: MCP Memo App
// Author: Rajeshwar Raja
// Date: 2025-12-28
// License: Proprietary

use anyhow::Result;
use serde::de::DeserializeOwned;
use reqwest::{header::CONTENT_TYPE, Client, Response, RequestBuilder};

pub mod service;

trait HttpServer {
    fn base_url(&self) -> &str;
    fn token(&self) -> &str;

    fn build_get_request(&self, endpoint: &str) -> RequestBuilder {
        let client = Client::new();
        client.get(format!("{}/{}", self.base_url(), endpoint))
            .header(CONTENT_TYPE, "application/json")
            .bearer_auth(self.token())
    }

    fn build_post_request(&self, endpoint: &str) -> RequestBuilder {
        let client = Client::new();
        client.post(format!("{}/{}", self.base_url(), endpoint))
            .header(CONTENT_TYPE, "application/json")
            .bearer_auth(self.token())
    }

    fn build_delete_request(&self, endpoint: &str) -> RequestBuilder {
        let client = Client::new();
        client.delete(format!("{}/{}", self.base_url(), endpoint))
            .header(CONTENT_TYPE, "application/json")
            .bearer_auth(self.token())
    }

    fn build_patch_request(&self, endpoint: &str) -> RequestBuilder {
        let client = Client::new();
        client.patch(format!("{}/{}", self.base_url(), endpoint))
            .header(CONTENT_TYPE, "application/json")
            .bearer_auth(self.token())
    }

    async fn validate_response(&self, rsp: Response) -> Result<()> {
        if !rsp.status().is_success() {
            let status = rsp.status();
            let text = rsp.text().await?;
            return Err(anyhow::anyhow!("Request failed: {} - {}", status, text));
        }
        Ok(())
    }
    async fn validate_data_response<T: DeserializeOwned>(&self, rsp: Response) -> Result<T> {
        if !rsp.status().is_success() {
            let status = rsp.status();
            let text = rsp.text().await?;
            return Err(anyhow::anyhow!("Request failed: {} - {}", status, text));
        }

        let data = rsp
            .json::<T>()
            .await?;

        Ok(data)
    }
}

pub struct Server {
    base_url: String,
    token: String,
    sign_out_required: bool,
}

impl Server {
    pub fn new(host: &str, token: &str) -> Self {
        Server {
            base_url: format!("http://{}/api/v1", host),
            token: token.to_string(),
            sign_out_required: false,
        }
    }

    pub async fn cleanup(&self) -> Result<()> {
        if self.sign_out_required {
            self.build_post_request("auth/signout")
                .send()
                .await?;
        }
        Ok(())
    }
}

impl HttpServer for Server {
    fn base_url(&self) -> &str {
        &self.base_url
    }

    fn token(&self) -> &str {
        &self.token
    }
}