<h1 align="center">Fire Auth</h1>  


<p align="center">
    <img src="https://raw.githubusercontent.com/nameshare/fireauth/master/logo.png" width="130" height="180">
</p>

<p align="center"><i>Rust wrapper for Firebase Authentication REST API</i></p>
<p align="center">
    <a href="https://crates.io/crates/fireauth/">
        <img src="https://img.shields.io/badge/crates.io-v0.1.2-green.svg" alt="WTFPL" />
    </a>
    <a href="http://www.wtfpl.net/">
        <img src="http://www.wtfpl.net/wp-content/uploads/2012/12/wtfpl-badge-4.png" width="85" height="20" alt="WTFPL" />
    </a>
</p>

## Installation
Add the following to Cargo.toml:

```toml
fireauth = "0.1.2"
```

## How to use

First you need to get a web `API_KEY` from [firebase project settings](https://console.firebase.google.com/project/_/settings/general/).

```rust
let api_key: String = "s6FqaFcRFd...njhB8cCjN7".to_owned();

let auth = fireauth::FireAuth::new(API_KEY);
```

<br/>

## Features
1. [Sign Up (email)](#1-sign-up-email)
2. [Sign In (email)](#2-sign-in-email)
3. [Send OOB Code](#3-send-oob-code)
4. [Refresh ID Token](#4-refresh-id-token)
5. [Get User Information](#5-get-user-information)
6. [Update Email and Password](#6-update-email-and-password)

> Don't see what you need? See below for [unsupported features for now](#what-are-not-supported-yet).

<br/>

### 1. Sign Up (email)

```rust
let email = "something@email.com".to_owned();
let password = "supersecret".to_owned();
let return_secure_token = true;

match auth.sign_up_email(email, password, return_secure_token).await {
    Ok(response) => ...,
    Err(error) => ...,
}

// response structure
pub struct fireauth::api::SignUpResponse {
    pub id_token: String,
    pub email: String,
    pub refresh_token: String,
    pub expires_in: String,
    pub local_id: String,
}
```

### 2. Sign In (email)
```rust
match auth.sign_in_email(email, password, return_secure_token).await {
    Ok(response) => ...,
    Err(error) => ...,
}

// response structure
pub struct fireauth::api::SignInResponse {
    pub kind: String,
    pub local_id: String,
    pub email: String,
    pub display_name: String,
    pub id_token: String,
    pub registered: bool,
    pub refresh_token: Option<String>,
    pub expires_in: Option<String>,
}
```

### 3. Send OOB Code
#### Send verification email
```rust
match auth.verify_email(id_token).await {
    Ok(send_oob_code) => ...
    Err(error) => ...
}

// response structure
pub struct fireauth::api::SendOobCode {
    pub kind: String,
    pub email: String,
}
```

#### Send reset password
```rust
match auth.reset_password(email).await {
    Ok(send_oob_code) => ...
    Err(error) => ...
}
```

### 4. Refresh ID Token
```rust
match auth.refresh_id_token(refresh_token).await {
    Ok(refresh_id_token_response) => ...
    Err(error) => ...
}

// response structure
pub struct fireauth::api::RefreshIdToken {
    pub access_token: String,
    pub expires_in: String,
    pub token_type: String,
    pub refresh_token: String,
    pub id_token: String,
    pub user_id: String,
    pub project_id: String,
}
```

### 5. Get User Information
```rust
match auth.get_user_info(id_token).await {
    Ok(user) => ...,
    Err(error) => ...,
}

// response structure
pub struct fireauth::api::User {
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

pub struct fireauth::api::ProviderUserInfo {
    pub provider_id: String,
    pub federated_id: String,
    pub email: String,
    pub raw_id: String,
}
```

### 6. Update Email and Password

#### Email
```rust
match auth.change_email(id_token, email, return_secure_token).await {
    Ok(update_user) => ...
    Err(error) => ...
}

// response structure
pub struct fireauth::api::UpdateUser {
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

pub struct fireauth::api::ProviderUserInfo {
    pub provider_id: String,
    pub federated_id: String,
    pub email: String,
    pub raw_id: String,
}
```

#### Password
```rust
match auth.change_password(id_token, password, return_secure_token).await {
    Ok(update_user) => ...
    Err(error) => ...
}
```

<br/>

## What are not supported yet

### Sign In
- Sign in anonymously
- Sign in with OAuth credential

### Password
- Verify password reset code
- Confirm password reset

### User
- Update profile
- Delete account
