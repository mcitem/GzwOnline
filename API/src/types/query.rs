use crate::define_query;
use crate::types::api::APIError;
use sea_orm::{query::Select, QuerySelect};
use serde_json::json;

use serde::Deserialize;
#[derive(Deserialize)]
pub struct Search {
    pub q: String,
}
define_query!(
    #[derive(Debug)]
    pub struct Pagination {
        order_by: Option<String>,
        offset: Option<u64>,
        limit: Option<u64>,
        page: Option<u64>,
        per_page: Option<u64>,
    }
);
pub fn pagination<T>(mut q: Select<T>, query: Pagination) -> Result<Select<T>, APIError>
where
    T: sea_orm::EntityTrait,
{
    if query.page.is_some() | query.per_page.is_some() {
        if let (Some(page), Some(per_page)) = (query.page, query.per_page) {
            let offset = (page - 1) * per_page;
            q = q.offset(offset).limit(per_page);
        } else {
            return Err(
                APIError::new_for_bad_request().err(json!("page and per_page must be provided"))
            );
        }
    } else if query.offset.is_some() {
        if let (Some(o), Some(l)) = (query.offset, query.limit) {
            q = q.offset(o).limit(l);
        } else {
            return Err(
                APIError::new_for_bad_request().err(json!("offset and limit must be provided"))
            );
        }
    } else if query.limit.is_some() {
        if let Some(l) = query.limit {
            q = q.limit(l);
        }
    }
    Ok(q)
}
