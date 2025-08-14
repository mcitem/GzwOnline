use crate::entities::collection;
use crate::entities::prelude::{Category, Collection, PhotoAlbum};
use crate::types::api::{APIError, EmptyDataType, API};
use crate::{types::*, OK};
use axum::{
    extract::{Path, Query, State},
    routing::get,
    Router,
};

use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QueryOrder, Set,
};
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/search", get(search))
        .route("/list", get(list))
        .route("/list/:id", get(list_with_id))
        .route("/:id/like", get(like))
        .route("/:id/photos", get(photo))
        .route("/:id", get(collection))
}

async fn like(Path(id): Path<i32>, State(state): State<AppState>) -> API {
    let collection = Collection::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or(APIError::new_for_404())?;
    let likes_count_d = collection.likes_count.unwrap_or(0) + 1;
    let mut collection: collection::ActiveModel = collection.into();
    collection.likes_count = Set((likes_count_d).into());
    collection.save(&state.db).await?;
    OK!(likes_count_d)
}

async fn search(Query(query): Query<query::Search>, State(state): State<AppState>) -> API {
    let collections = Collection::find()
        .filter(collection::Column::Title.contains(query.q))
        .all(&state.db)
        .await?;
    OK!(collections)
}

async fn list(Query(query): Query<query::Pagination>, State(state): State<AppState>) -> API {
    let q = Collection::find();
    let q = match &query.order_by {
        Some(order_by) => match order_by.as_str() {
            "LikesCount" => q.order_by_desc(collection::Column::LikesCount),
            "ViewsCount" => q.order_by_desc(collection::Column::ViewsCount),
            "CategoryId" => q.order_by_asc(collection::Column::CategoryId),
            _ => q,
        },
        None => q,
    };
    let collections = query::pagination::<Collection>(q, query)?;
    let collections = collections.all(&state.db).await?;
    OK!(collections)
}

async fn list_with_id(
    Path(id): Path<i32>,
    Query(query): Query<query::Pagination>,
    State(state): State<AppState>,
) -> API {
    if id == 0 {
        return list(Query(query), State(state)).await;
    };
    let q = Category::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or(APIError::new_for_200(EmptyDataType::EmptyArray))?;
    let q = q.find_related(Collection);
    let q = query::pagination::<Collection>(q, query)?;
    let collections = q.all(&state.db).await?;
    OK!(collections)
}

async fn collection(Path(id): Path<i32>, State(state): State<AppState>) -> API {
    let o_collection = Collection::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or(APIError::new_for_404())?;

    let c = o_collection.views_count.unwrap_or(0) + 1;
    let mut collection: collection::ActiveModel = o_collection.clone().into();
    collection.views_count = Set((c).into());
    collection.save(&state.db).await?;

    OK!(o_collection)
}

async fn photo(Path(id): Path<i32>, State(state): State<AppState>) -> API {
    let collection = Collection::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or(APIError::new_for_200(EmptyDataType::EmptyArray))?;
    let photos = collection
        .find_related(PhotoAlbum)
        .all(&state.db)
        .await?
        .into_iter()
        .filter_map(|v| v.photo)
        .collect::<Vec<_>>();
    OK!(photos)
}
