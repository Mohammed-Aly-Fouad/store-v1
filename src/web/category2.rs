use axum::{
    Json, Router, extract::{Form, Path, Query, State}, http::StatusCode, response::{IntoResponse, Redirect}, routing::{get, post},
};
use serde::Deserialize;

use crate::domain::category::dto2::{
    CategoryResponseDto, CategoryRow, CategoryTemplate, CreateCategoryForm,
};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(render_categories_page))
        // .route("/category/fetch/{id}", get(fetch_category_by_id))
        // .route("/", post(create_category_web)) // or combine on the same path if applicable
        // .route("/edit/{id}", get(edit_category_page))
        // .route("/update/{id}", post(update_category_web))
        // .route("/delete/{id}", post(delete_category_web))
        // .route("/search", get(search_categories_handler))
}