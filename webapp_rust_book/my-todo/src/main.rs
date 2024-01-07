use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());

    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    let app = create_app();
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    tracing::debug!("listebing on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}

fn create_app() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
}

async fn root() -> &'static str {
    "Hello, world"
}

async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct CreateUser {
    username: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct User {
    id: u64,
    username: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use axum::{
        body::Body,
        http::{header, Method, Request},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn should_return_hello_world() {
        let req = Request::builder().uri("/").body(Body::empty()).unwrap();
        let res = create_app().oneshot(req).await.unwrap();

        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body: String = String::from_utf8((bytes.to_vec())).unwrap();

        assert_eq!(body, "Hello, world");
    }

    // #[tokio::test]
    // async fn should_return_user_data() {
    //     let req: Request<Body> = Request::builder()
    //         .uri("/users")
    //         .method(Method::POST)
    //         .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
    //         .body(Body::from(r#"{"username: "田中 太郎"}"#))
    //         .unwrap();

    //     let res = create_app().oneshot(req).await.unwrap();

    //     let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
    //     let body: String = String::from_utf8(bytes.to_vec()).unwrap();

    //     let user: User = serde_json::from_str(&body).expect("cannot convert User instnce.");

    //     assert_eq!(
    //         user,
    //         User {
    //             id: 1337,
    //             username: "田中 太郎".to_string(),
    //         }
    //     );
    // }
}

// user repository

use anyhow::Context;
use thiserror::Error;

#[derive(Debug, Error)]
enum RepositoryError {
    #[error("NotFound, id is {0}")]
    NotFound(i32),
}

pub trait TodoRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    fn create(&self, payload: CreateTodo) -> Todo;
    fn find();
    fn all();
    fn update();
    fn delete();
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Todo {
    id: i32,
    text: String,
    completed: bool,
}

pub struct CreateTodo {
    text: String,
}
