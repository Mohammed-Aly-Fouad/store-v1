use core::sync;

use axum::Router;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::routing::get;
use serde::Deserialize;

use crate::domain::category::dto::{CategoryCreateTemplate, CategoryResponseDTO, CategoryTemplate, CreateCategoryForm};
use crate::state::AppState;




pub fn router() -> Router<AppState> {
    Router::new()
    .route("/", get(render_categories_page))
    .route("/create", get(render_create_page))
    
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
let categories = fetch_all_categories(&state).await;
let (main_categories, sub_categories): (Vec<CategoryResponseDTO>, Vec<CategoryResponseDTO>) = categories.iter().cloned()
.partition(|category| category.parent_id.is_none());
    CategoryTemplate {
        categories: categories,
        main_categories: main_categories,
        sub_categories: sub_categories,
        error_message: error_message,
        success_message: success_message,
        current_page: "categories".to_string(),
    }
}



pub async fn render_create_page(
    State(state): State<AppState>, // 1. استقبال State للوصول للـ Database
) -> impl IntoResponse {
    // 2. جلب وتصفية الفئات الرئيسية فقط (التي ليس لها parent_id)
    let main_categories: Vec<CategoryResponseDTO> = fetch_all_categories(&state)
        .await
        .into_iter()
        .filter(|c| c.parent_id.is_none())
        .collect();

    CategoryCreateTemplate {
        form: CreateCategoryForm::default(),
        main_categories, // 3. تمرير القائمة إلى الـ Template
        current_page: "categories".to_string(),
        error_message: None,
        success_message: None,
        errors: None,
    }
}



#[derive(Debug, Deserialize)]
pub struct FlashParams {
    pub action: Option<String>,
    pub error: Option<String>
}