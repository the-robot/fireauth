use crate::{error::Error};
use serde::{Serialize, Deserialize};
use super::FailResponse;

impl crate::FireAuth {
    pub async fn sign_up_email(&self, email: String, password: String, return_secure_token: bool) -> Result<Response, Error> {
        let url = format!(
            "https://identitytoolkit.googleapis.com/v1/accounts:signUp?key={}",
            self.api_key,
        );

        let client = reqwest::Client::new();
        let resp = client.post(&url)
            .header("Content-Type", "application/json")
            .json(&RequestPayload{
                email,
                password,
                return_secure_token
            })
            .send()
            .await?;

        if resp.status() != 200 {
            let error = resp.json::<FailResponse>().await?.error;
            return Err(Error::SignUp(error.message));
        }
        let body = resp.json::<Response>().await?;
        Ok(body)
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct RequestPayload {
    email: String,
    password: String,
    return_secure_token: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id_token: String,
    pub email: String,
    pub refresh_token: String,
    pub expires_in: String,
    pub local_id: String,
}