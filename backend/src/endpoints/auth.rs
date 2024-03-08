use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing, Router,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

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
pub struct SignUp {
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

pub async fn find_one(
    state: &AppState,
    username: &str,
    password: &str,
) -> anyhow::Result<Option<User>> {
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

async fn signin(State(state): State<AppState>, Json(data): Json<NewUser>) -> ApiResponse {
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
        data.password,
        data.email,
        data.name,
        data.surname,
    )
    .fetch_one(&state.db)
    .await?;
    Ok(Json(result).into_response())
}

async fn signup(State(state): State<AppState>, Json(data): Json<SignUp>) -> ApiResponse {
    let user = find_one(&state, &data.username, &data.password).await?;
    if user.is_none() {
        return Ok(StatusCode::UNAUTHORIZED.into_response());
    }
    let token = "".to_string();
    Ok(token.into_response())
}
