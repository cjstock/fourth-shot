use std::sync::Arc;

use askama::Template;
use axum::response::{Html, IntoResponse, Response};
use reqwest::StatusCode;

use crate::cdrag::CDragon;

pub mod champions;

#[derive(Debug, Clone)]
pub struct AppState {
    pub cdrag: Arc<CDragon>,
}

impl AppState {
    pub async fn new() -> anyhow::Result<AppState> {
        let cdrag = CDragon::new().await?;
        Ok(AppState {
            cdrag: Arc::new(cdrag),
        })
    }
}

#[derive(Debug, displaydoc::Display, thiserror::Error)]
pub enum AppError {
    /// could not render template
    Render(#[from] askama::Error),
    /// anyhow support
    Anyhow(#[from] anyhow::Error),
    /// missing
    NotFound,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        #[derive(Debug, Template)]
        #[template(path = "error.html")]
        struct Tmpl {}

        let status = match &self {
            AppError::Render(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound => StatusCode::NOT_FOUND,
        };
        let tmpl = Tmpl {};
        if let Ok(body) = tmpl.render() {
            (status, Html(body)).into_response()
        } else {
            (status, "Something went wrong").into_response()
        }
    }
}
