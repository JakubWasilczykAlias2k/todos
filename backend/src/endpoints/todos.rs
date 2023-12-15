use std::fs::read_to_string;

use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
    routing, Router,
};
use serde::{Deserialize, Serialize};

const URL: &str = "/todos";
const URL_ID: &str = "/todos/:id";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToDo {
    id: u64,
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

pub fn router() -> Router {
    Router::new()
        // `GET /` goes to `root`
        .route(URL, routing::get(get_all))
        .route(URL, routing::post(post))
        .route(URL_ID, routing::get(get))
        .route(URL_ID, routing::put(put))
        .route(URL_ID, routing::patch(patch))
        .route(URL_ID, routing::delete(delete))
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

async fn get_all() -> impl IntoResponse {
    let array = read_file().unwrap();
    Json(array)
}
async fn post(Json(data): Json<NewToDo>) -> impl IntoResponse {
    let mut array = read_file().unwrap();
    let id = array.iter().map(|x| x.id).max().unwrap_or(0) + 1;
    let item = ToDo {
        id,
        title: data.title,
        description: data.description,
        completed: data.completed,
    };
    array.push(item.clone());
    write_file(array).unwrap();
    Json(item).into_response()
}
async fn get(Path(id): Path<u64>) -> impl IntoResponse {
    let array = read_file().unwrap();
    let item = array.iter().find(|x| x.id == id);
    match item {
        Some(x) => Json(x).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}
async fn put(Path(id): Path<u64>, Json(data): Json<NewToDo>) -> impl IntoResponse {
    let mut array = read_file().unwrap();
    let index = array.iter().position(|x| x.id == id);
    if index.is_none() {
        return StatusCode::NOT_FOUND.into_response();
    }
    let item = ToDo {
        id,
        title: data.title,
        description: data.description,
        completed: data.completed,
    };
    array[index.unwrap()] = item.clone();
    write_file(array).unwrap();
    Json(item).into_response()
}
async fn patch(Path(id): Path<u64>, Json(data): Json<EditToDo>) -> impl IntoResponse {
    let mut array = read_file().unwrap();
    let index = array.iter().position(|x| x.id == id);
    if index.is_none() {
        return StatusCode::NOT_FOUND.into_response();
    }
    let item = &array[index.unwrap()];
    let item = ToDo {
        id: item.id,
        title: data.title.unwrap_or(item.title.clone()),
        description: data.description.unwrap_or(item.description.clone()),
        completed: data.completed.unwrap_or(item.completed),
    };
    array[index.unwrap()] = item.clone();
    write_file(array).unwrap();
    Json(item).into_response()
}
async fn delete(Path(id): Path<u64>) -> impl IntoResponse {
    let mut array = read_file().unwrap();
    let index = array.iter().position(|x| x.id == id);
    if index.is_none() {
        return StatusCode::NOT_FOUND.into_response();
    }
    array.remove(index.unwrap());
    write_file(array).unwrap();
    StatusCode::NO_CONTENT.into_response()
}
