use axum::{response::IntoResponse, routing::get, Json, Router};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn axum_simple_sample() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[tokio::main]
async fn axum_service_sample() {
    // our router
    let app = Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(create_user))
        .route("/foo/bar", get(foo_bar));
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
// which calls one of these handlers
async fn root() -> &'static str {
    "Hello, World!"
}

async fn get_foo() -> impl IntoResponse {
    let user = User {
        id: 1337,
        username: "Wee".to_string(),
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}
async fn post_foo() {}
async fn foo_bar() {}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_axum_sample_main() {
        axum_simple_sample();
    }

    #[ignore]
    #[test]
    fn test_axum_service_sample_main() {
        axum_service_sample();
    }
}
