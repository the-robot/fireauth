mod sign_up;
mod sign_in;
mod user;
mod token;

pub use sign_in::{Response as SignInResponse};
pub use sign_up::{Response as SignUpResponse};
pub use user::{User, UpdateUser, ProviderUserInfo, SendOobCode};
pub use token::{RefreshToken};

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct FailResponse {
    error: FailResponseBody
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct FailResponseBody {
    // code: u16,
    message: String,
}
