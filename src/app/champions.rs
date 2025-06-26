use askama::Template;
use axum::{
    Router,
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
};

use super::AppError;
use super::AppState;

pub fn router<S>(state: AppState) -> Router<S> {
    Router::new()
        .route("/champions", get(champions_grid))
        .with_state(state)
}

struct ChampionGridItem {
    name: String,
    icon_url: String,
}

#[derive(Template)]
#[template(path = "champions_grid.html")]
struct ChampionsGridTemplate {
    champions: Vec<ChampionGridItem>,
}

async fn champions_grid(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let champ_objs: Vec<ChampionGridItem> = state
        .cdrag
        .champions
        .keys()
        .into_iter()
        .filter_map(|id| {
            state.cdrag.champion_by_id(*id).map(|ch| ChampionGridItem {
                name: ch.name.clone(),
                icon_url: ch.square_portrait_path.clone(),
            })
        })
        .collect();

    let template = ChampionsGridTemplate {
        champions: champ_objs,
    };

    Ok(Html(template.render()?))
}
