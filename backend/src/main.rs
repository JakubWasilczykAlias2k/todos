mod endpoints;
use std::fs::OpenOptions;

use axum::Router;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let _ = OpenOptions::new().create(true).open("todos.json");

    // build our application with a route
    let app = Router::new().merge(endpoints::todos::router());
    // `GET /` goes to `root`

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind("127.0.0.1:6969")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
