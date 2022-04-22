use axum::extract::Form;
use axum::response::Html;
use axum::routing::get;
use axum::Router;
use serde::Deserialize;

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

#[derive(Deserialize, Debug)]
struct Input {
    name: String
}

async fn sub_post_handler(Form(input): Form<Input>) {
    dbg!(&input);
}
