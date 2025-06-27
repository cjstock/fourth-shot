use std::collections::HashMap;

use askama::Template;
use axum::{
    Router,
    extract::{Query, State},
    response::{Html, IntoResponse},
    routing::get,
};
use chrono::NaiveDate;

use crate::cdrag::Champion;

use super::AppError;
use super::AppState;

pub fn router<S>(state: AppState) -> Router<S> {
    Router::new()
        .route("/champions", get(champions_grid))
        .with_state(state)
}

struct ChampionGridItem {
    id: u64,
    name: String,
    icon_url: String,
}

#[derive(Template)]
#[template(path = "champions_grid.html")]
struct ChampionsGridTemplate {
    champions: Vec<ChampionGridItem>,
}

async fn champions_grid(
    State(state): State<AppState>,
    Query(search_params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, AppError> {
    let mut champ_objs: Vec<ChampionGridItem> = state
        .cdrag
        .champions
        .values()
        .map(|ch| ChampionGridItem {
            id: ch.id,
            name: ch.name.clone(),
            icon_url: ch.square_portrait_path.clone(),
        })
        .collect();

    let sort_by = search_params.get("sort_by").map_or("name", |s| s.as_str());
    let sort_order = search_params
        .get("sort_order")
        .map_or("asc", |s| s.as_str());

    match (sort_by, sort_order) {
        ("name", "asc") => champ_objs.sort_by(|a, b| a.name.cmp(&b.name)),
        ("name", "desc") => champ_objs.sort_by(|a, b| b.name.cmp(&a.name)),
        _ => champ_objs.sort_by(|a, b| a.name.cmp(&b.name)), // Default sort
    }

    let template = ChampionsGridTemplate {
        champions: champ_objs,
    };

    Ok(Html(template.render()?))
}
