mod endpoints;
use std::fs::OpenOptions;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Router,
};
use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};
use tower_http::cors::CorsLayer;

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Sqlite>,
}

pub type ApiResponse = Result<Response, ApiError>;
pub struct ApiError(anyhow::Error);

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Oops somethings went wrong! {:?}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for ApiError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to load .env file");
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

    // initialize tracing
    tracing_subscriber::fmt::init();

    let _ = OpenOptions::new().create(true).open("todos.json");

    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        Sqlite::create_database(&db_url)
            .await
            .expect("Error while creating database");
    }

    let db = SqlitePool::connect(&db_url)
        .await
        .expect("Unable to connect to the database");

    // run migrations
    sqlx::migrate!()
        .run(&db)
        .await
        .expect("Failed to run migrations");

    let state = AppState { db };

    // build our application with a route
    let app = Router::<AppState>::new()
        .merge(endpoints::todos::router())
        .merge(endpoints::auth::router())
        .layer(CorsLayer::permissive())
        .with_state(state);
    // `GET /` goes to `root`

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind("127.0.0.1:6969")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
