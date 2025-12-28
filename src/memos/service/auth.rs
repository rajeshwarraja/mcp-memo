// Project: MCP Memo App
// Author: Rajeshwar Raja
// Date: 2025-12-28
// License: Proprietary

use anyhow::Result;
use serde::{Serialize, Deserialize};
use crate::memos::Server;

#[derive(Debug, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "ROLE_UNSPECIFIED")]
    RoleUnspecified,
    #[serde(rename = "HOST")]
    Host,
    #[serde(rename = "ADMIN")]
    Admin,
    #[serde(rename = "USER")]
    User,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "STATE_UNSPECIFIED")]
    StateUnspecified,
    #[serde(rename = "NORMAL")]
    Normal,
    #[serde(rename = "ARCHIVED")]
    Archived,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(default)] pub name: String,
    pub role: Role,
    pub username: String,
    #[serde(default)] pub email: String,
    #[serde(default, rename = "displayName")] pub display_name: String,
    #[serde(default, rename = "avatarUrl")] pub avatar_url: String,
    #[serde(default)] pub description: String,
    pub state: State,
}

pub trait AuthService {
    async fn get_current_user(&self) -> Result<User>;

    async fn sign_in(&self, username: &str, password: &str) -> Result<Server>;
}

impl<T> AuthService for T where T: crate::memos::HttpServer {
    async fn get_current_user(&self) -> Result<User> {
        let rsp = self.build_get_request("auth/me")
            .send()
            .await?;

        #[derive(Deserialize)]
        struct ResponseBody {
            pub user: User
        }

        Ok(self.validate_data_response::<ResponseBody>(rsp).await?.user)
    }

    async fn sign_in(&self, username: &str, password: &str) -> Result<Server> {
        #[derive(Serialize)]
        struct PasswordCredentials<'a> {
            username: &'a str,
            password: &'a str,
        }
        #[derive(Serialize)]
        struct RequestBody<'a> {
            #[serde(rename = "passwordCredentials")]
            password_credentials: PasswordCredentials<'a>,
        }
        let body = RequestBody {
            password_credentials: PasswordCredentials {
                username,
                password,
            },
        };

        let rsp = self.build_post_request("auth/signin")
            .json(&body)
            .send()
            .await?;

        #[derive(Deserialize)]
        struct ResponseBody {
            #[serde(rename = "accessToken")] pub access_token: String,
        }

        let data = self.validate_data_response::<ResponseBody>(rsp).await?;

        Ok(Server {
            base_url: self.base_url().to_string(),
            token: data.access_token,
            sign_out_required: true
        })
    }
}