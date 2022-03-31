use crate::{error::Error};
use serde::{Serialize, Deserialize};
use super::FailResponse;

impl crate::FireAuth {
    pub async fn refresh_token(&self, refresh_token: String) -> Result<RefreshToken, Error> {
        let url = format!(
            "https://securetoken.googleapis.com/v1/token?key={}",
            self.api_key,
        );

        let client = reqwest::Client::new();
        let resp = client.post(url)
            .header("Content-Type", "application/json")
            .json(&RefreshTokenPayload {
                grant_type: "refresh_token".to_owned(),
                refresh_token,
            })
            .send()
            .await?;

        if resp.status() != 200 {
            let error = resp.json::<FailResponse>().await?.error;
            return Err(Error::Token(error.message));
        }

        let body = resp.json::<RefreshToken>().await?;
        Ok(body)
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct RefreshTokenPayload {
    grant_type: String,
    refresh_token: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshToken {
    pub access_token: String,
    pub expires_in: String,
    pub token_type: String,
    pub refresh_token: String,
    pub id_token: String,
    pub user_id: String,
    pub project_id: String,
}
