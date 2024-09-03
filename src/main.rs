#![allow(dead_code)]
#![allow(unused_imports)]
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Extension, Router,
};
use std::sync::Arc;
use tera::{Context, Tera};
use tower_http::services::ServeDir;
use tracing::{debug, error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let tera = match Tera::new("templates/**/*") {
        Ok(t) => Arc::new(t),
        Err(e) => {
            println!("Error parsing templates: {}", e);
            std::process::exit(1)
        }
    };

    let app = Router::new()
        .route("/", get(home))
        .route("/greet/:name", get(greet))
        .layer(Extension(tera))
        .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Listing on {}", listener.local_addr().unwrap());
    info!("Server started at {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn greet(
    Path(name): Path<String>,
    Extension(tera): Extension<Arc<Tera>>,
) -> impl IntoResponse {
    let mut context = Context::new();
    context.insert("name", &name);
    tera.render("hello.html", &context).map(Html).map_err(|e| {
        error!("Erorr rendering template: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

async fn home(Extension(tera): Extension<Arc<Tera>>) -> impl IntoResponse {
    let context = Context::new();
    tera.render("home.html", &context).map(Html).map_err(|e| {
        error!("Error rendering template: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}
