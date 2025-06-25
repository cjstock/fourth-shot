use axum::{Router, extract::State, response::IntoResponse, routing::get};

use super::AppError;
use super::AppState;
use crate::cdrag;
use crate::cdrag::Champion;

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

struct ChampionsGribTemplate {
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
                id: *id,
                name: ch.name.clone(),
                icon_url: ch.square_portrait_path.clone(),
            })
        })
        .collect();

    Ok("Done")
}
