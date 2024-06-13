use axum::{
    routing::get,
    Router,
};
use axum::response::Html;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async {
        Html(r#"
        <style>
        body {
            background-color: black;
            color: green;`
        }
        </style>

        <body>
            <h1>Deltric's Rad Website</h1>
            <h2>Subtext</h2>
        </body>
        "#)
    }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
