use axum::extract::Form;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::{Json, Router};
use sub::{Request, Response};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .route("/sub", get(sub_handler).post(sub_post_handler));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html(include_str!("resources/index.html"))
}

async fn sub_handler() -> Html<&'static str> {
    Html(include_str!("resources/sub.html"))
}

async fn sub_post_handler(Form(input): Form<Request>) -> impl IntoResponse {
    let response = Response::new("OK");
    Json(response)
}
