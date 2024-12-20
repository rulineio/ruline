use std::{sync::Arc, time::Duration};

use crate::Result;
use reqwest_middleware::ClientWithMiddleware;
use serde::Deserialize;

use super::{error::ClientError, new_client};

pub struct Client {
    client: Arc<ClientWithMiddleware>,
    client_id: String,
    client_secret: String,
}

impl Client {
    pub fn new(client_id: String, client_secret: String) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(ClientError::Reqwest)?;

        Ok(Self {
            client: Arc::new(new_client(client)),
            client_id,
            client_secret,
        })
    }

    pub fn get_oauth_url(&self, request: &OAuthRequest) -> String {
        format!(
            "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&scope={}&access_type={}&login_hint={}&state={}&response_type={}",
            self.client_id,
            request.redirect_uri,
            request.scope,
            request.access_type,
            request.login_hint,
            request.state,
            request.response_type
        )
    }

    pub async fn get_access_token(&self, code: String, redirect_uri: String) -> Result<String> {
        let response = self
            .client
            .post("https://oauth2.googleapis.com/token")
            .form(&[
                ("client_id", &self.client_id),
                ("client_secret", &self.client_secret),
                ("code", &code),
                ("grant_type", &"authorization_code".to_owned()),
                ("redirect_uri", &redirect_uri),
            ])
            .send()
            .await
            .map_err(ClientError::ReqwestMiddleware)?;

        if !response.status().is_success() {
            return Err(ClientError::UnexpectedStatus(response.status()).into());
        }

        let response: OAuthTokenResponse = response.json().await.map_err(ClientError::Reqwest)?;
        Ok(response.access_token)
    }

    pub async fn get_user_info(&self, access_token: String) -> Result<UserInfoResponse> {
        let response = self
            .client
            .get("https://www.googleapis.com/oauth2/v1/userinfo")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await
            .map_err(ClientError::ReqwestMiddleware)?;

        if !response.status().is_success() {
            return Err(ClientError::UnexpectedStatus(response.status()).into());
        }

        let response: UserInfoResponse = response.json().await.map_err(ClientError::Reqwest)?;
        Ok(response)
    }
}

pub struct OAuthRequest {
    pub state: String,
    pub redirect_uri: String,
    pub scope: String,
    pub access_type: String,
    pub login_hint: String,
    pub response_type: String,
}

#[derive(Deserialize)]
pub struct OAuthTokenResponse {
    pub access_token: String,
}

#[derive(Debug, Deserialize)]
pub struct UserInfoResponse {
    pub id: String,
    pub email: String,
    pub verified_email: bool,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
}
