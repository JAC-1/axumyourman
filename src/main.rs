#![allow(unused)] // For beginners? why I am a beginnner !

use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Extension, Router,
};
use std::sync::Arc;
use tera::{Context, Tera};

fn render_template(
    tera: &Tera,
    template_name: &str,
    context: &Context,
) -> Result<Html<String>, (StatusCode, &'static str)> {
    tera.render(template_name, context)
        .map(Html)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))
}
#[tokio::main]
async fn main() {
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => Arc::new(t),
        Err(e) => {
            println!("Error parsing templates: {}", e);
            std::process::exit(1)
        }
    };
    let app = Router::new()
        .route("/", get(welcome))
        .route("/greet/:name", get(greet))
        .layer(Extension(tera));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Listing on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn greet(
    Path(name): Path<String>,
    Extension(tera): Extension<Arc<Tera>>,
) -> impl IntoResponse {
    let mut context = Context::new();
    context.insert("name", &name);

    render_template(&tera, "hello.html", &context)
}

async fn welcome(Extension(tera): Extension<Arc<Tera>>) -> impl IntoResponse {
    let mut context = Context::new();
    render_template(&tera, "welcome.html", &context)
}
