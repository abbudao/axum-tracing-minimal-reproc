use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "tracing_error_reproc=trace,tower_http=trace,axum::rejection=trace"
                    .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new().route("/", get(handler))
    .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[tracing::instrument]
fn add_one(a: i32) -> i32 {
    a + 1
}

#[tracing::instrument]
async fn handler() -> Html<&'static str> {
    add_one(1);
    Html("<h1>Hello, World!</h1>")
}
