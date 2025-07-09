use crate::cdrag::Skin;
use anyhow::anyhow;
use std::collections::HashMap;

use askama::Template;
use axum::{
    Router,
    extract::{Path, Query, State},
    response::{Html, IntoResponse},
    routing::get,
};

use super::AppError;
use super::AppState;

pub fn router<S>(state: AppState) -> Router<S> {
    Router::new()
        .route("/champions", get(champions_grid))
        .route("/champions/{id}", get(champion_detail))
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
    let search_term = search_params
        .get("search_term")
        .map_or("", |s| s.as_str())
        .to_lowercase();
    let mut champ_objs: Vec<ChampionGridItem> = state
        .cdrag
        .champions
        .values()
        .filter(|ch| ch.name.to_lowercase().contains(&search_term))
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

struct ChampionDetail {
    id: u64,
    name: String,
    title: String,
    short_bio: String,
    skins: Vec<Skin>,
}

impl ChampionDetail {
    pub fn base_skin(&self) -> Option<&Skin> {
        self.skins.iter().find(|skin| skin.is_base)
    }

    pub fn skins_no_base(&self) -> Vec<&Skin> {
        self.skins.iter().filter(|skin| !skin.is_base).collect()
    }
}

#[derive(Template)]
#[template(path = "champion_detail.html")]
struct ChampionDetailTemplate {
    champion: ChampionDetail,
}

async fn champion_detail(
    State(state): State<AppState>,
    Path(champion_id): Path<u64>,
) -> Result<impl IntoResponse, AppError> {
    let champion = state.cdrag.champion_by_id(champion_id);
    match champion {
        None => Err(AppError::NotFound),
        Some(champ) => {
            

            let skins = champ.skins.clone();
            return Ok(Html(
                ChampionDetailTemplate {
                    champion: ChampionDetail {
                        id: champ.id,
                        name: champ.name.clone(),
                        title: champ.title.clone(),
                        short_bio: champ.short_bio.clone(),
                        skins,
                    },
                }
                .render()?,
            ));
        }
    }
}
