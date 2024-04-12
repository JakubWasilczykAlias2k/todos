use axum::{extract::Request, middleware::Next};

use crate::ApiResponse;

pub async fn auth(request: Request, next: Next) -> ApiResponse {
    println!("Auth middleware");
    Ok(next.run(request).await)
}
