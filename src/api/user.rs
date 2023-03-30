use crate::{error::Error};
use serde::{Serialize, Deserialize};
use super::FailResponse;

impl crate::FireAuth {
    pub async fn get_user_info(&self, id_token: &str) -> Result<User, Error> {
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
        &self, id_token: &str, email: &str, return_secure_token: bool,
    ) -> Result<UpdateUser, Error> {
        self.update_user(id_token, Some(email), None, return_secure_token).await
    }

    pub async fn change_password(
        &self, id_token: &str, password: &str, return_secure_token: bool,
    ) -> Result<UpdateUser, Error> {
        self.update_user(id_token, None, Some(password), return_secure_token).await
    }

    pub async fn reset_password(&self, email: &str) -> Result<SendOobCode, Error> {
        self.send_oob_code("PASSWORD_RESET", None, Some(email)).await
    }

    pub async fn verify_email(&self, id_token: &str) -> Result<SendOobCode, Error> {
        self.send_oob_code("VERIFY_EMAIL", Some(id_token), None).await
    }

    async fn update_user(
        &self, id_token: &str, email: Option<&str>, password: Option<&str>, return_secure_token: bool,
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

    async fn send_oob_code(&self, request_type: &str, id_token: Option<&str>, email: Option<&str>) -> Result<SendOobCode, Error> {
        let url = format!(
            "https://identitytoolkit.googleapis.com/v1/accounts:sendOobCode?key={}",
            self.api_key,
        );

        let client = reqwest::Client::new();
        let resp = client.post(url)
            .header("Content-Type", "application/json")
            .json(&SendOobCodePayload { request_type, id_token, email })
            .send()
            .await?;

        if resp.status() != 200 {
            let error = resp.json::<FailResponse>().await?.error;
            return Err(Error::User(error.message));
        }

        let body = resp.json::<SendOobCode>().await?;
        Ok(body)
    }
}

// User Info
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct UserInfoPayload<'a> {
    id_token: &'a str,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UserInfoResponse {
    // kind: String,
    users: Vec<User>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub local_id: String,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub display_name: Option<String>,
    pub provider_user_info: Vec<ProviderUserInfo>,
    pub photo_url: Option<String>,
    pub password_hash: Option<String>,
    pub password_updated_at: Option<u64>,
    pub valid_since: String,
    pub disabled: Option<bool>,
    pub last_login_at: String,
    pub created_at: String,
}

// Change Email/Password
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateUserPayload<'a> {
    id_token: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<&'a str>,

    return_secure_token: bool,
}

#[derive(Debug, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderUserInfo {
    pub provider_id: String,
    pub federated_id: String,
    pub email: Option<String>,
    pub raw_id: String,
}

// Email Verification
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SendOobCodePayload<'a> {
    request_type: &'a str,
    id_token: Option<&'a str>,
    email: Option<&'a str>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendOobCode {
    pub kind: String,
    pub email: String,
}
