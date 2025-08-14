use crate::entities::feedback::ActiveModel;
use crate::types::api::API;
use crate::types::AppState;
use crate::OK;
use axum::extract::Json;
use axum::extract::State;
use axum::routing::post;
use axum::routing::Router;
use sea_orm::ActiveModelTrait;
use sea_orm::Set;
use serde::Deserialize;
#[derive(Deserialize)]
pub struct Feedback {
    pub contact: Option<String>,
    pub content: String,
}

pub fn router() -> Router<AppState> {
    Router::new().route("/new", post(new))
}

async fn new(State(state): State<AppState>, Json(fb): Json<Feedback>) -> API {
    let ac = ActiveModel {
        content: Set(Some(fb.content)),
        contact: Set(fb.contact),
        ..Default::default()
    };
    ac.insert(&state.db).await?;
    OK!("")
}
