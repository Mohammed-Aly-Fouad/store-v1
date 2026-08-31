
use axum::extract::{Path, Query, State};
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::{get, post};
use axum::{Form, Router};

use crate::domain::category::dto::{
    CategoryCreateTemplate, CategoryFormErrors, CategoryResponseDTO, CategoryTemplate, CategoryForm, CategoryEditTemplate, FlashParams
};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(render_categories_page))
        .route("/create", get(render_create_page))
        .route("/{id}/edit", get(render_edit_page))
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
    let (main_categories, _) = get_main_and_sub_categories(&state)
        .await
        .unwrap_or_default();
// tracing::info!("main_categories:\n{main_categories:#?}");
    CategoryCreateTemplate {
        main_categories,
        form: CategoryForm::default(),
        errors: None,
        error_message: None,
        success_message: None,
        current_page: "categories".to_string(),
    }
}










pub async fn create_category(
    State(state): State<AppState>,
    Form(mut form): Form<CategoryForm>,
) -> Response {
    let (main_categories, _) = match get_main_and_sub_categories(&state).await {
        Ok(data) => data,
        Err(err) => {
            tracing::error!("فشل جلب الفئات: {:?}", err);
            return CategoryCreateTemplate {
                main_categories: vec![],
                form,
                errors: None,
                error_message: Some("حدث خطأ في جلب البيانات".to_string()),
                success_message: None,
                current_page: "categories".to_string(),
            }
            .into_response();
        }
    };

    form.sanitize();

    if let Err(form_err) = form.validate() {
        return CategoryCreateTemplate {
            main_categories,
            form,
            errors: Some(form_err),
            error_message: Some("يرجى تصحيح الأخطاء لإستكمال التسجيل".to_string()),
            success_message: None,
            current_page: "categories".to_string(),
        }
        .into_response();
    }

    // تحقق من وجود الأب فعليًا لو المستخدم كتب اسم
    let parent_id: Option<i64> = if let Some(ref name) = form.parent_name {
        match sqlx::query_scalar!(
            "SELECT id FROM categories WHERE name_ar = $1 OR name_en = $1 LIMIT 1",
            name
        )
        .fetch_optional(&state.pool)
        .await
        {
            Ok(id) => id,
            Err(err) => {
                tracing::error!("فشل البحث عن الفئة الأب: {:?}", err);
                None
            }
        }
    } else {
        None
    };

    if form.parent_name.is_some() && parent_id.is_none() {
        let mut errors = CategoryFormErrors::default();
        errors.parent_name = Some("الفئة الأب غير موجودة".to_string());
        return CategoryCreateTemplate {
            main_categories,
            form,
            errors: Some(errors),
            error_message: Some("يرجى تصحيح الأخطاء لإستكمال التسجيل".to_string()),
            success_message: None,
            current_page: "categories".to_string(),
        }
        .into_response();
    }

    let result = sqlx::query!(
        r#"
        INSERT INTO categories (name_en, name_ar, parent_id, notes)
        VALUES ($1, $2, $3, $4)
        "#,
        form.name_en,
        form.name_ar,
        parent_id,
        form.notes
    )
    .execute(&state.pool)
    .await;

    match result {
        Ok(_) => Redirect::to("/web/categories?action=created").into_response(),
        Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23505") => {
            CategoryCreateTemplate {
                main_categories,
                form,
                errors: None,
                error_message: Some("هذا البيان مسجل بالفعل".to_string()),
                success_message: None,
                current_page: "categories".to_string(),
            }
            .into_response()
        }
        Err(err) => {
            tracing::error!("فشل إدخال الفئة في قاعدة البيانات: {:?}", err);
            CategoryCreateTemplate {
                main_categories,
                form,
                errors: None,
                error_message: Some("حدث خطأ عام .. برجاء المحاولة لاحقاً".to_string()),
                success_message: None,
                current_page: "categories".to_string(),
            }
            .into_response()
        }
    }
}





pub async fn render_edit_page(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Response {
     let (main_categories, sub_categories) = get_main_and_sub_categories(&state)
        .await
        .unwrap_or_default();
    let category = sqlx::query_as!(
        CategoryResponseDTO,
        r#" SELECT
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
WHERE c.id = $1;"#,
id
    ).fetch_optional(&state.pool)
    .await;

    match category {
        Ok(Some(c)) => {
            let form = CategoryForm {
                name_en: c.name_en.clone(),
                name_ar: c.name_ar.clone(),
                parent_name: c.parent_name.clone(),
                notes: c.notes.clone(),
            };

            CategoryEditTemplate {
                main_categories,
                sub_categories,
                category: Some(c),
                form,
                errors: None,
                success_message: None,
                error_message: None,
                current_page: "categories".to_string(),
            }.into_response()
        }

        Ok(None) => Redirect::to("/web/categories/error=not_found").into_response(),
        Err(_) => Redirect::to("/web/categories/error=db_err").into_response()
        
    }
}