use core::sync;

use axum::extract::{Path, Query, State};
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::{get, post};
use axum::{Form, Router};
use serde::Deserialize;
use sqlx::encode::IsNull::No;

use crate::domain::category::dto::{
    CategoryCreateTemplate, CategoryResponseDTO, CategoryTemplate, CreateCategoryForm,
};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(render_categories_page))
        .route("/create", get(render_create_page))
    .route("/", post(create_category))
}

async fn get_main_and_sub_categories(
    state: &AppState,
) -> Result<(Vec<CategoryResponseDTO>, Vec<CategoryResponseDTO>), sqlx::Error> {
    let categories = sqlx::query_as!(
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
    .unwrap_or_default();

    Ok(categories
        .into_iter()
        .partition(|category| category.parent_id.is_none()))
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
    let (main_categories, sub_categories) = get_main_and_sub_categories(&state)
        .await
        .unwrap_or_default();
    CategoryTemplate {
        main_categories: main_categories,
        sub_categories: sub_categories,
        error_message: error_message,
        success_message: success_message,
        current_page: "categories".to_string(),
    }
}

pub async fn render_create_page(State(state): State<AppState>,) -> impl IntoResponse {
    let (main_categories, sub_categories) = get_main_and_sub_categories(&state)
        .await
        .unwrap_or_default();
    CategoryCreateTemplate {
        main_categories,
        sub_categories,
        form: CreateCategoryForm::default(),
        errors: None,
        error_message: None,
        success_message: None,
        current_page: "brand".to_string(),
    }
}

#[derive(Debug, Deserialize)]
pub struct FlashParams {
    pub action: Option<String>,
    pub error: Option<String>,
}


pub async fn create_category(
    State(state): State<AppState>,
    Form(mut form): Form<CreateCategoryForm>,
) -> Response {
    let (main_categories, sub_categories) = get_main_and_sub_categories(&state)
        .await
        .unwrap_or_default();
    if let Err(form_err) = form.validate() {
        return CategoryCreateTemplate {
             main_categories,
        sub_categories,
            form,
            errors: Some(form_err),
            error_message: Some("يرجى تصحيح الأخطاء لإستكمال التسجيل".to_string()),
            success_message: None,
            current_page: "brand".to_string(), 
        }
        .into_response();
    }

    form.name_en = form.name_en.trim().to_string();
    form.name_ar = form.name_ar.trim().to_string();
    form.parent_name = form
        .parent_name
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from); 
    form.notes = form
        .notes
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from);

    let result = sqlx::query!(
        r#"
    INSERT INTO categories (name_en, name_ar, parent_id, notes)
    VALUES (
        $1, 
        $2, 
        (SELECT id FROM categories WHERE name_ar = $3 OR name_en = $3 LIMIT 1), 
        $4
    )
    "#,
        form.name_en,
        form.name_ar,
        form.parent_name,
        form.notes
    )
    .execute(&state.pool)
    .await;

    match result {
        Ok(_) => Redirect::to("/web/categories?action=created").into_response(),
        Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23505") => {
           CategoryCreateTemplate {
             main_categories,
        sub_categories,
             form,
            errors: None,
            error_message: Some("هذا البيان مسجل بالفعل".to_string()),
                success_message: None,
                current_page: "categories".to_string(),
           } 
           .into_response()
        }

         Err(err) => {
            tracing::error!("فشل إدخال العلامة التجارية في قاعدة البيانات: {:?}", err);

            CategoryCreateTemplate {
                 main_categories,
        sub_categories,
                form,
                errors: None,
                error_message: Some("حدث خطأ عام .. برجاء المحاولة لاحقاً".to_string()),
                success_message: None,
                current_page: "brand".to_string(),
            }
            .into_response()
        }
    }
}


