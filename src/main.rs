use anyhow::Context;
use askama::Template;
use axum::{
    Router,
    extract::{MatchedPath, Query},
    http::Request,
    response::{Html, IntoResponse},
    routing::get,
};
use std::collections::HashMap;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::info_span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use fourth_shot::app::{AppError, AppState};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/", get(hello))
        .route("/hello", get(say_hello))
        .with_state(AppState::new())
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                info_span!(
                    "http_request",
                        method = ?request.method(),
                        matched_path,
                        some_other_field = tracing::field::Empty,
                )
            }),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .with_context(|| "Failed to open a TCP connection")?;
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .with_context(|| "Failed to server the app")?;
    Ok(())
}

async fn say_hello(
    Query(params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Html(
        HelloTemplate {
            name: params.get("name").unwrap().to_owned(),
        }
        .render()?,
    ))
}

#[derive(Template)]
#[template(source = "{{name}}", ext = "html")]
struct HelloTemplate {
    name: String,
}

#[derive(Template)]
#[template(path = "hello.html")]
struct IndexTemplate {
    name: String,
}

async fn hello() -> Result<impl IntoResponse, AppError> {
    Ok(Html(
        IndexTemplate {
            name: "Corey".to_string(),
        }
        .render()?,
    ))
}
