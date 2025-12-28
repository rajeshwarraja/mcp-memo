// Project: MCP Memo App
// Author: Rajeshwar Raja
// Date: 2025-12-28
// License: Proprietary

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub last_used_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "ROLE_UNSPECIFIED")]
    RoleUnspecified,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(default)] pub name: String,
    pub role: Role,
    pub username: String,
    #[serde(default)] pub email: String,
    #[serde(default, rename = "displayname")] pub display_name: String,
    #[serde(default, rename = "avatarUrl")] pub avatar_url: String,
    #[serde(default)] pub description: String,
    #[serde(default)] pub password: String,
    pub state: State,
}

impl User {
    pub fn new(username: &str, password: &str, email: &str) -> Self {
        User {
            name: "".to_string(),
            role: Role::User,
            username: username.to_string(),
            email: email.to_string(),
            display_name: String::new(),
            avatar_url: String::new(),
            description: String::new(),
            password: password.to_string(),
            state: State::Normal,
        }
    }
}

pub trait UserService {
    async fn create_user(&self, user: &User) -> Result<User>;

    async fn delete_user(&self, user: &User) -> Result<()>;

    async fn create_pat(&self, user: &User, desc: &str, expires_in_days: u32) -> Result<(Token, String)>;

    async fn delete_pat(&self, token: &Token) -> Result<()>;
}


impl<T> UserService for T
where
    T: crate::memos::HttpServer,
{
    async fn create_user(&self, user: &User) -> Result<User> {
        let request = self.build_post_request("users")
            .json(user);

        let response = request.send().await?;

        let created_user = self.validate_data_response::<User>(response).await?;

        Ok(created_user)
    }

    async fn delete_user(&self, user: &User) -> Result<()> {
        let endpoint = format!("{}", user.name);
        let request = self.build_delete_request(&endpoint);

        let response = request.send().await?;

        self.validate_response(response).await?;

        Ok(())
    }

    async fn create_pat(&self, user: &User, desc: &str, expires_in_days: u32) -> Result<(Token, String)> {
        #[derive(Serialize)]
        struct RequestBody {
            parent: String,
            description: String,
            #[serde(rename = "expiresInDays")]
            expires_in_days: u32,
        }

        let body = RequestBody {
            parent: user.name.clone(),
            description: desc.to_string(),
            expires_in_days,
        };

        let endpoint = format!("{}/personalAccessTokens", user.name);
        let rsp = self.build_post_request(&endpoint)
            .json(&body)
            .send()
            .await?;
        
        #[derive(Deserialize)]
        struct ResponseData {
            #[serde(rename = "personalAccessToken")] pub personal_access_token: Token,
            pub token: String,
        }
        
        let data = self.validate_data_response::<ResponseData>(rsp).await?;
        Ok((data.personal_access_token, data.token))
    }

    async fn delete_pat(&self, token: &Token) -> Result<()> {
        let endpoint = format!("{}", token.name);
        let rsp = self.build_delete_request(&endpoint)
            .send()
            .await?;

        self.validate_response(rsp).await?;

        Ok(())
    }

}

#[cfg(test)]
mod tests {
    use super::{*, super::{auth::AuthService, super::Server}};
    
    #[tokio::test]
    async fn test_create_and_delete_user() {
        let server = Server::new("localhost:5230", "memos_pat_t3pjYKgGSzYqOqMgR4mZR768afCNG6sW");
        let user = User::new("testuser", "testpassword", "test@example.com");
        let created_user = server.create_user(&user).await.expect("Failed to create user");
        assert_eq!(created_user.username, user.username);

        server.delete_user(&created_user).await.expect("Failed to delete user");
    }

    #[tokio::test]
    async fn test_create_and_delete_pat() {
        let server = Server::new("localhost:5230", "memos_pat_t3pjYKgGSzYqOqMgR4mZR768afCNG6sW");
        let user = User::new("testuser2", "testpassword2", "test2@example.com");
        let created_user = server.create_user(&user).await.expect("Failed to create user");
        {
            let server = server.sign_in("testuser2", "testpassword2").await.expect("Failed to sign in");
            let (token, plain_text) = server.create_pat(&created_user, "Test PAT", 30).await.expect("Failed to create PAT");
            assert_eq!(token.description, "Test PAT");
            assert!(!plain_text.is_empty());

            server.delete_pat(&token).await.expect("Failed to delete PAT");
        }
        server.delete_user(&created_user).await.expect("Failed to delete user");
    }
}