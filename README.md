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

<br/>

## Features
1. [Sign Up (email)](#1-sign-up-email)
2. [Sign In (email)](#2-sign-in-email)
3. [Send OOB Code](#3-send-oob-code)
4. [Refresh ID Token](#4-refresh-id-token)
5. [Get User Information](#5-get-user-information)
6. [Update Email and Password](#6-update-email-and-password)

<br/>

### 1. Sign Up (email)

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

### 2. Sign In (email)
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

### 3. Send OOB Code
#### Send verification email
```rust
match fireauth.verify_email(id_token).await {
    Ok(send_oob_code) => ...
    Err(error) => ...
}

// response structure
struct SendOobCode {
    kind: String,
    email: String,
}
```

#### Send reset password
```rust
match fireauth.reset_password(email).await {
    Ok(send_oob_code) => ...
    Err(error) => ...
}
```

### 4. Refresh ID Token
```rust
match fireauth.refresh_id_token(refresh_token).await {
    Ok(refresh_id_token_response) => ...
    Err(error) => ...
}

// response structure
struct RefreshIdToken {
    access_token: String,
    expires_in: String,
    token_type: String,
    refresh_token: String,
    id_token: String,
    user_id: String,
    project_id: String,
}
```

### 5. Get User Information
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

### 6. Update Email and Password

#### Email
```rust
match fireauth.change_email(id_token, email, false).await {
    Ok(update_user) => ...
    Err(error) => ...
}

// response structure
struct UpdateUser {
    kind: String,
    local_id: String,
    email: String,
    provider_user_info: Vec<ProviderUserInfo>,
    password_hash: String,
    email_verified: bool,
    id_token: Option<String>,
    refresh_token: Option<String>,
    expires_in: Option<String>,
}

struct ProviderUserInfo {
    provider_id: String,
    federated_id: String,
    email: String,
    raw_id: String,
}
```

#### Password
```rust
match fireauth.change_password(id_token, password, true).await {
    Ok(update_user) => ...
    Err(error) => ...
}
```
