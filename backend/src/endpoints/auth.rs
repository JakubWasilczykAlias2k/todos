use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing, Router,
};
use chrono::{Local, NaiveDateTime, TimeDelta, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};

use crate::{ApiResponse, AppState};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    id: i64,
    username: String,
    email: String,
    name: String,
    surname: String,
    created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    username: String,
    password: String,
    email: String,
    name: String,
    surname: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignIn {
    username: String,
    password: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        // `GET /` goes to `root`
        .route("/auth/signin", routing::post(signin))
        .route("/auth/userinfo", routing::get(userinfo))
        .route("/auth/signup", routing::post(signup))
}

fn hash_password(password: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(password);

    let password = hasher.finalize();
    format!("{:x}", password)
}

pub async fn find_one(
    state: &AppState,
    username: &str,
    password: &str,
) -> anyhow::Result<Option<User>> {
    let password = hash_password(password);
    let result = sqlx::query_as!(
        User,
        r#"SELECT 
            id,
            username,
            email,
            name,
            surname,
            created_at
        FROM users 
        WHERE username = $1 AND password = $2"#,
        username,
        password
    )
    .fetch_optional(&state.db)
    .await?;
    Ok(result)
}

async fn userinfo(State(state): State<AppState>) -> ApiResponse {
    let result = sqlx::query_as!(
        User,
        r#"SELECT 
            id,
            username,
            email,
            name,
            surname,
            created_at
        FROM users"#
    )
    .fetch_all(&state.db)
    .await?;
    Ok(Json(result).into_response())
}

async fn signup(State(state): State<AppState>, Json(data): Json<NewUser>) -> ApiResponse {
    let password = hash_password(&data.password);

    let result = sqlx::query_as!(
        User,
        r#"INSERT INTO users (username, password, email, name, surname) VALUES ($1, $2, $3, $4, $5) RETURNING 
            id,
            username,
            email,
            name,
            surname,
            created_at
        "#,
        data.username,
        password,
        data.email,
        data.name,
        data.surname,
    )
    .fetch_one(&state.db)
    .await?;
    Ok(Json(result).into_response())
}

async fn signin(State(state): State<AppState>, Json(data): Json<SignIn>) -> ApiResponse {
    let user = find_one(&state, &data.username, &data.password).await?;
    if user.is_none() {
        return Ok(StatusCode::UNAUTHORIZED.into_response());
    }
    let user = user.unwrap();
    let mut hasher = Sha512::new();
    hasher.update(user.username);
    hasher.update(user.name);
    hasher.update(user.id.to_ne_bytes());
    hasher.update(user.surname);
    hasher.update(user.email);
    let now = Local::now().to_string();
    hasher.update(now);

    let token = hasher.finalize();
    let token = format!("{:x}", token);

    let expires_at = Utc::now() + TimeDelta::try_days(1).unwrap();
    sqlx::query!(
        r#"INSERT INTO sessions (user_id, token, expires_at) VALUES ($1, $2, $3)"#,
        user.id,
        token,
        expires_at,
    )
    .execute(&state.db)
    .await?;

    Ok(token.into_response())
}
