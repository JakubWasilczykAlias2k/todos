use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/stocazzo", get(root2));

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind("127.0.0.1:6969")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn root2() -> &'static str {
    "Hello, Stocazzo!"
}
