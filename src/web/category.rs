use core::sync;

use axum::Router;
use axum::extract::{Path, Query, State};
use axum::routing::get;
use serde::Deserialize;

use crate::domain::category::dto::{CategoryResponseDTO, CategoryTemplate};
use crate::state::AppState;




pub fn router() -> Router<AppState> {
    Router::new()
    .route("/", get(render_categories_page))
    
}

async fn fetch_all_categories(state: &AppState) -> Vec<CategoryResponseDTO> {
    sqlx::query_as!(
        CategoryResponseDTO,
        r#"
        SELECT
            c.id,
            c.name_en,
            c.name_ar,
            c.parent_id,
            p.name_ar AS "parent_name?",
            c.notes,
            c.created_at,
            c.updated_at
        FROM categories c
        LEFT JOIN categories p ON c.parent_id = p.id
        ORDER BY c.id DESC;
        "#
    )
    .fetch_all(&state.pool)
    .await
    .unwrap_or_default()
}

pub async fn render_categories_page(
    State(state): State<AppState>,
    Query(params): Query<FlashParams>,
) -> CategoryTemplate {
    let success_message = match params.action.as_deref() {
        Some("created") => Some("تم إضافة الفئة بنجاح".to_string()),
        Some("updated") => Some("تم تعديل الفئة بنجاح".to_string()),
        Some("deleted") => Some("تم حذف الفئة بنجاح".to_string()),
        _ => None,
    };

    let error_message = match params.error.as_deref() {
        Some("not_found") => Some("غير موجود بقاعدة البيانات".to_string()),
        Some("db_error") => Some("خطأ عام بقاعدة البيانات".to_string()),
        _ => None,
    };

    CategoryTemplate {
        categories: fetch_all_categories(&state).await,
        error_message: error_message,
        success_message: success_message,
        current_page: "categories".to_string(),
    }
}







#[derive(Debug, Deserialize)]
pub struct FlashParams {
    pub action: Option<String>,
    pub error: Option<String>
}