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

enum Sort {
    NameAsc,
    NameDesc,
}

impl<'a> From<&'a str> for Sort {
    fn from(value: &'a str) -> Self {
        match value {
            "nameDesc" => Sort::NameDesc,
            _ => Sort::NameAsc,
        }
    }
}

async fn champions_grid(
    State(state): State<AppState>,
    Query(search_params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, AppError> {
    let sort = search_params.get("sort");

    let sort: Sort = sort.map_or(Sort::NameAsc, |it| it.as_str().into());

    let filters = search_params.get("filters");
    let search = search_params.get("search");

    let mut champ_objs: Vec<ChampionGridItem> = state
        .cdrag
        .champions
        .keys()
        .into_iter()
        .filter_map(|id| {
            state.cdrag.champion_by_id(*id).map(|ch| ChampionGridItem {
                id: ch.id,
                name: ch.name.clone(),
                icon_url: ch.square_portrait_path.clone(),
            })
        })
        .collect();
    champ_objs.sort_by(|a, b| a.name.cmp(&b.name));

    let template = ChampionsGridTemplate {
        champions: champ_objs,
    };

    Ok(Html(template.render()?))
}
