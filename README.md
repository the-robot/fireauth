<h1 align="center">Fire Auth</h1>  


<p align="center">
    <img src="https://raw.githubusercontent.com/nameshare/fireauth/master/logo.png" width="130" height="180">
</p>

<p align="center"><i>Rust wrapper for Firebase Authentication REST API</i></p>

## How to use

First you need to get `API_KEY` from [project_settings](https://console.firebase.google.com/project/_/settings/general/).

```rust
const API_KEY: String = "s6FqaFcRFd...njhB8cCjN7".to_owned();

let fireauth = fireauth::FireAuth::new(API_KEY);
```

### Sign Up (email)

```rust
let email = "something@email.com".to_owned();
let password = "supersecret".to_owned();
let return_secure_token = true;

match fireauth.sign_up_email(email, password, return_secure_token).await {
    Ok(response) => ...,
    Err(error) => ...,
}

// response structure
struct SignUpResponse {
    id_token: String,
    email: String,
    refresh_token: String,
    expires_in: String,
    local_id: String,
}
```

### Sign In (email)
```rust
match fireauth.sign_in_email(email, password, return_secure_token).await {
    Ok(response) => ...,
    Err(error) => ...,
}

// response structure
struct SignInResponse {
    kind: String,
    local_id: String,
    email: String,
    display_name: String,
    id_token: String,
    registered: bool,
    refresh_token: Option<String>,
    expires_in: Option<String>,
}
```

### Get User Information
```rust
match fireauth.get_user_info(id_token).await {
    Ok(user) => ...,
    Err(error) => ...,
}

// response structure
struct User {
    local_id: String,
    email: String,
    password_hash: String,
    email_verified: bool,
    password_updated_at: u64,
    provider_user_info: Vec<ProviderUserInfo>,
    valid_since: String,
    last_login_at: String,
    created_at: String,
    last_refresh_at: String,
}

struct ProviderUserInfo {
    provider_id: String,
    federated_id: String,
    email: String,
    raw_id: String,
}
```
