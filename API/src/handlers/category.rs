use crate::entities::{category, collection};
use crate::types::api::API;
use crate::types::*;
use crate::{
    entities::prelude::{Category, Collection},
    OK,
};
use api::{APIError, EmptyDataType};
use axum::extract::Path;
use axum::{extract::State, routing::get, Router};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/list", get(list))
        .route("/collections", get(collections))
        .route("/collections/:id", get(collections_with_id))
}

async fn list(State(state): State<AppState>) -> API {
    let categories = Category::find().all(&state.db).await?;
    OK!(categories)
}
use serde::Serialize;
#[derive(Debug, Serialize)]
struct Tabbar {
    pub id: i32,
    pub name: String,
    pub foods: Vec<collection::Model>,
}

async fn collections(State(state): State<AppState>) -> API {
    let categories_with_collections = Category::find()
        .find_with_related(Collection)
        .all(&state.db)
        .await?;
    let uncategorized_collections = Collection::find()
        .filter(collection::Column::CategoryId.is_null())
        .all(&state.db)
        .await?;
    let mut data = Vec::new();
    data.push(Tabbar {
        id: 0,
        name: "未分类".into(),
        foods: uncategorized_collections,
    });
    for (category, collections) in categories_with_collections {
        data.push(Tabbar {
            id: category.id,
            name: category.name,
            foods: collections,
        });
    }

    OK!(data)
}

async fn collections_with_id(State(state): State<AppState>, Path(id): Path<i32>) -> API {
    let category = match id {
        0 => category::Model {
            id: 0,
            name: "未分类".into(),
        },
        _ => Category::find()
            .filter(category::Column::Id.eq(id))
            .one(&state.db)
            .await?
            .ok_or(APIError::new_for_200(EmptyDataType::EmptyObject))?,
    };
    let collections = Collection::find()
        .filter(match id {
            0 => collection::Column::CategoryId.is_null(),
            _ => collection::Column::CategoryId.eq(id),
        })
        .all(&state.db)
        .await?;
    let data = Tabbar {
        id,
        name: category.name,
        foods: collections,
    };
    OK!(data)
}
