use crate::{error::Error};
use serde::{Serialize, Deserialize};
use super::FailResponse;

impl crate::FireAuth {
    pub async fn get_user_info(&self, id_token: String) -> Result<User, Error> {
        let url = format!(
            "https://identitytoolkit.googleapis.com/v1/accounts:lookup?key={}",
            self.api_key,
        );

        let client = reqwest::Client::new();
        let resp = client.post(url)
            .header("Content-Type", "application/json")
            .json(&UserInfoPayload { id_token })
            .send()
            .await?;

        if resp.status() != 200 {
            let error = resp.json::<FailResponse>().await?.error;
            return Err(Error::User(error.message));
        }

        let body = resp.json::<UserInfoResponse>().await?;
        Ok(body.users[0].clone())
    }

    pub async fn change_email(
        &self, id_token: String, email: String, return_secure_token: bool,
    ) -> Result<UpdateUser, Error> {
        self.update_user(id_token, Some(email), None, return_secure_token).await
    }

    pub async fn change_password(
        &self, id_token: String, password: String, return_secure_token: bool,
    ) -> Result<UpdateUser, Error> {
        self.update_user(id_token, None, Some(password), return_secure_token).await
    }

    async fn update_user(
        &self, id_token: String, email: Option<String>, password: Option<String>, return_secure_token: bool,
    ) -> Result<UpdateUser, Error> {
        let url = format!(
            "https://identitytoolkit.googleapis.com/v1/accounts:update?key={}",
            self.api_key,
        );

        let client = reqwest::Client::new();
        let resp = client.post(url)
            .header("Content-Type", "application/json")
            .json(&UpdateUserPayload {
                id_token,
                email,
                password,
                return_secure_token,
            })
            .send()
            .await?;

        if resp.status() != 200 {
            let error = resp.json::<FailResponse>().await?.error;
            return Err(Error::User(error.message));
        }

        let body = resp.json::<UpdateUser>().await?;
        Ok(body)
    }
}

// User Info
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct UserInfoPayload {
    id_token: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UserInfoResponse {
    // kind: String,
    users: Vec<User>
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub local_id: String,
    pub email: String,
    pub password_hash: String,
    pub email_verified: bool,
    pub password_updated_at: u64,
    pub provider_user_info: Vec<ProviderUserInfo>,
    pub valid_since: String,
    pub last_login_at: String,
    pub created_at: String,
    pub last_refresh_at: String,
}

// Change Email/Password
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateUserPayload {
    id_token: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<String>,

    return_secure_token: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUser {
    pub kind: String,
    pub local_id: String,
    pub email: String,
    pub provider_user_info: Vec<ProviderUserInfo>,
    pub password_hash: String,
    pub email_verified: bool,
    pub id_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_in: Option<String>,
}

// Provider User Info
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderUserInfo {
    pub provider_id: String,
    pub federated_id: String,
    pub email: String,
    pub raw_id: String,
}
