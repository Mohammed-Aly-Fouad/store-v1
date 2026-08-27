use axum::{
    Json, Router, extract::{Form, Path, Query, State}, http::StatusCode, response::{IntoResponse, Redirect}, routing::{get, post},
};
use serde::Deserialize;

use crate::domain::category::dto2::{
    CategoryResponseDto, CategoryRow, CategoryTemplate, CreateCategoryForm,
};
use crate::domain::category::dto::FlashParams;
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

async fn fetch_all_categories(state: &AppState) -> Vec<CategoryResponseDto> {
    sqlx::query_as::<_, CategoryResponseDto>(
        "SELECT id, name_en, name_ar, parent_id, notes, created_at, updated_at
         FROM categories ORDER BY id DESC",
    )
    .fetch_all(&state.pool)
    .await
    .unwrap_or_default()
}

fn build_root_categories(
    categories: &[CategoryResponseDto],
    exclude_id: Option<i64>,
) -> Vec<CategoryResponseDto> {
    categories
        .iter()
        .filter(|c| c.parent_id.is_none() && Some(c.id) != exclude_id)
        .cloned()
        .collect()
}

fn is_unique_violation(err: &sqlx::Error) -> bool {
    matches!(err, sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505"))
}

fn is_foreign_key_violation(err: &sqlx::Error) -> bool {
    matches!(err, sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23503"))
}

pub async fn render_categories_page(
    State(state): State<AppState>,
    Query(params): Query<FlashParams>,
) -> CategoryTemplate {
    let success_message = match params.action.as_deref() {
        Some("created") => Some("تم إنشاء الفئة بنجاح".to_string()),
        Some("updated") => Some("تم تحديث الفئة بنجاح".to_string()),
        Some("deleted") => Some("تم حذف الفئة بنجاح".to_string()),
        _ => None,
    };

    let all = fetch_all_categories(&state).await;
    println!("{:?}", all);
    CategoryTemplate {
        root_categories: build_root_categories(&all, None),
        categories: CategoryRow::build_rows(&all),
        error_message: None,
        success_message,
        current_page: "categories".to_string(),
        
    }
}