use std::sync::Arc;

use askama::Template;
use axum::response::{Html, IntoResponse, Response};
use reqwest::StatusCode;

use crate::cdrag::CDragon;

mod champions;

#[derive(Default, Debug, Clone)]
pub struct AppState {
    pub cdrag: Arc<CDragon>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState::default()
    }
}

#[derive(Debug, displaydoc::Display, thiserror::Error)]
pub enum AppError {
    /// could not render template
    Render(#[from] askama::Error),
    /// anyhow support
    Anyhow(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        #[derive(Debug, Template)]
        #[template(path = "error.html")]
        struct Tmpl {}

        let status = match &self {
            AppError::Render(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let tmpl = Tmpl {};
        if let Ok(body) = tmpl.render() {
            (status, Html(body)).into_response()
        } else {
            (status, "Something went wrong").into_response()
        }
    }
}
