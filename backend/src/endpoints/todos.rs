use std::fs::read_to_string;

use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing, Router,
};
use serde::{Deserialize, Serialize};

use crate::{ApiResponse, AppState};

const URL: &str = "/todos";
const URL_ID: &str = "/todos/:id";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToDo {
    id: i64,
    title: String,
    description: String,
    completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewToDo {
    title: String,
    description: String,
    completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditToDo {
    title: Option<String>,
    description: Option<String>,
    completed: Option<bool>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        // `GET /` goes to `root`
        .route(URL, routing::get(get_all))
        .route(URL, routing::post(post))
        .route(URL_ID, routing::get(get))
        .route(URL_ID, routing::put(put))
        .route(URL_ID, routing::patch(patch))
        .route(URL_ID, routing::delete(delete))
}

pub async fn find_one(state: &AppState, id: i64) -> anyhow::Result<Option<ToDo>> {
    let result = sqlx::query_as!(ToDo, "SELECT * FROM todos WHERE id = $1", id)
        .fetch_optional(&state.db)
        .await?;
    Ok(result)
}

pub fn read_file() -> anyhow::Result<Vec<ToDo>> {
    let file_contents = read_to_string("todos.json")?;
    let list = serde_json::from_str::<Vec<ToDo>>(&file_contents)?;
    Ok(list)
}

pub fn write_file(list: Vec<ToDo>) -> anyhow::Result<()> {
    let file_contents = serde_json::to_string(&list)?;
    std::fs::write("todos.json", file_contents)?;
    Ok(())
}

async fn get_all(State(state): State<AppState>) -> ApiResponse {
    let result = sqlx::query_as!(ToDo, "SELECT * FROM todos")
        .fetch_all(&state.db)
        .await?;
    Ok(Json(result).into_response())
}

async fn post(State(state): State<AppState>, Json(data): Json<NewToDo>) -> ApiResponse {
    let result = sqlx::query_as!(
        ToDo,
        "INSERT INTO todos (title, description, completed) VALUES ($1, $2, $3) RETURNING *",
        data.title,
        data.description,
        data.completed,
    )
    .fetch_one(&state.db)
    .await?;
    Ok(Json(result).into_response())
}

async fn get(State(state): State<AppState>, Path(id): Path<i64>) -> ApiResponse {
    match find_one(&state, id).await? {
        Some(x) => Ok(Json(x).into_response()),
        None => Ok(StatusCode::NOT_FOUND.into_response()),
    }
}

async fn put(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(data): Json<NewToDo>,
) -> ApiResponse {
    let result = find_one(&state, id).await?;
    if result.is_none() {
        return Ok(StatusCode::NOT_FOUND.into_response());
    }
    let _ = sqlx::query!(
        "UPDATE todos SET title = $1, description = $2, completed = $3 WHERE id = $4",
        data.title,
        data.description,
        data.completed,
        id
    )
    .execute(&state.db)
    .await?;
    let result = ToDo {
        id,
        title: data.title,
        description: data.description,
        completed: data.completed,
    };
    Ok(Json(result).into_response())
}

async fn patch(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(data): Json<EditToDo>,
) -> ApiResponse {
    let result = find_one(&state, id).await?;
    if result.is_none() {
        return Ok(StatusCode::NOT_FOUND.into_response());
    }
    let result = result.unwrap();
    let title = data.title.unwrap_or(result.title);
    let description = data.description.unwrap_or(result.description);
    let completed = data.completed.unwrap_or(result.completed);
    let _ = sqlx::query!(
        "UPDATE todos SET title = $1, description = $2, completed = $3 WHERE id = $4",
        title,
        description,
        completed,
        id
    )
    .execute(&state.db)
    .await?;
    let result = ToDo {
        id,
        title,
        description,
        completed,
    };
    Ok(Json(result).into_response())
}

async fn delete(State(state): State<AppState>, Path(id): Path<i64>) -> ApiResponse {
    let result = find_one(&state, id).await?;
    if result.is_none() {
        return Ok(StatusCode::NOT_FOUND.into_response());
    }
    let _ = sqlx::query!("DELETE FROM todos WHERE id = $1", id)
        .execute(&state.db)
        .await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}
