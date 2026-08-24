
use std::result;

use axum::{
    extract::{Form, Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use sqlx::encode::IsNull::No;

use crate::domain::brand::dto::{BrandCreateTemplate, BrandFormErrors, BrandResponseDTO, BrandUpdateTemplate, BrandsTemplate, CreateBrandForm, FlashParams};

use crate::state::AppState;



/// Configures and returns the sub-router for all browser-based Askama HTML endpoints.
pub fn router() -> Router<AppState> {
    Router::new()
    .route("/", get(render_brands_page))
    .route("/", post(create_brand))
    .route("/create", get(render_form_page))
    .route("/create", post(create_brand))
    .route("/edit/{id}", get(render_edit_page))
    .route("/edit/{id}", post(edit_brand))

    }


pub async fn render_form_page() -> impl IntoResponse {
    BrandCreateTemplate {
        form: CreateBrandForm::default(),
        errors: None,
        error_message: None,
        success_message: None,
        current_page: "brand".to_string(),
    }
}


    async fn fetch_all_brands(state: &AppState) -> Vec<BrandResponseDTO> {
    sqlx::query_as!(
        BrandResponseDTO,
        r#"
        SELECT id, name_en, name_ar, notes, created_at, updated_at 
        FROM brands 
        ORDER BY id DESC
        "#
    )
    .fetch_all(&state.pool)
    .await
    .unwrap_or_default()
}

/// يجلب براند واحد عن طريق الـ `ID` لتحميل بياناته مسبقاً في نموذج التعديل.
async fn fetch_brand_by_id(state: &AppState, id: i64) -> Option<BrandResponseDTO> {
    sqlx::query_as!(
        BrandResponseDTO,
        r#"
        SELECT id, name_en, name_ar, notes, created_at, updated_at 
        FROM brands 
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(&state.pool)
    .await
    .unwrap_or(None)
}






// ============================================================================
// DATABASE HELPER FUNCTIONS
// ============================================================================

/// يجلب كافة البراندات مرتبة تنازلياً حسب المعرف (`ID`).
/// 
/// في حالة حدوث خطأ في قاعدة البيانات، تُرجع الدالة قائمة فارغة لضمان
/// استمرارية عمل واجهة المستخدم وعدم توقف الصفحة بالكامل.


// ============================================================================
// HANDLERS: READ & RENDER
// ============================================================================

/// Renders the main HTML page containing the brand list and creation form.
///
/// Checks for flash params (`?action=...`) in the query string to display contextual 
/// success notifications after a redirect.
pub async fn render_brands_page(
    State(state): State<AppState>,
    Query(params): Query<FlashParams>,
) -> BrandsTemplate {
    let success_message = match params.action.as_deref() {
        Some("created") => Some("تم إضافة البراند بنجاح".to_string()),
        Some("updated") => Some("تم تعديل البراند بنجاح".to_string()),
        Some("deleted") => Some("تم حذف البراند بنجاح".to_string()),
        _ => None,
    };

    BrandsTemplate {
        brands: fetch_all_brands(&state).await,
        error_message: None,
        success_message,
        current_page: "brands".to_string(),
        // brand_form_errors: None,
        // edit_brand: None,
        // form_data: None,
        // show_modal: false,
    }
}




pub async fn create_brand(
    State(state): State<AppState>,
    Form(form): Form<CreateBrandForm>,
) -> Response {
    // 1. التحقق من المدخلات (Validation)
    if let Err(form_err) = form.validate() {
        return BrandCreateTemplate {
            form,
            errors: Some(form_err),
            error_message: Some("يرجى تصحيح الأخطاء لإستكمال التسجيل".to_string()),
            success_message: None,
            current_page: "brand".to_string(),
        }
        .into_response();
    }

    let trimmed_name_en = form.name_en.trim();
    let trimmed_name_ar = form.name_ar.trim();
    let trimmed_notes = form
        .notes
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from);

    // 2. التنفيذ في قاعدة البيانات
    let result = sqlx::query!(
        r#"INSERT INTO brands (name_en, name_ar, notes) VALUES ($1, $2, $3)"#,
        trimmed_name_en,
        trimmed_name_ar,
        trimmed_notes
    )
    .execute(&state.pool)
    .await;

    // 3. معالجة النتيجة
    match result {
        Ok(_) => Redirect::to("/web/brands?action=created").into_response(),

        // خطأ التكرار Unique Constraint
        Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23505") => {
            BrandCreateTemplate {
                form,
                errors: None,
                error_message: Some("هذا البيان مسجل بالفعل".to_string()),
                success_message: None,
                current_page: "brand".to_string(),
            }
            .into_response()
        }

        // خطأ عام من قاعدة البيانات
        Err(_) => BrandCreateTemplate {
            form, // تم إضافتها حتى لا يتوقف التجميع
            errors: None,
            error_message: Some("حدث خطأ عام .. برجاء المحاولة لاحقاً".to_string()),
            success_message: None,
            current_page: "brand".to_string(),
        }
        .into_response(),
    }
}


pub async fn render_edit_page(
    State(state): State<AppState>,
    Path(id): Path<i64>
) -> Response {
    let brand = sqlx::query_as!(
        BrandResponseDTO,
        r#"SELECT id, name_en, name_ar, notes, created_at, updated_at FROM brands WHERE id = $1"#,
        id
    ).fetch_optional(&state.pool)
    .await;

    match brand {
        Ok(Some(b)) => {
            let form = CreateBrandForm {
                name_en: b.name_en.clone(),
                name_ar: b.name_ar.clone(),
                notes: b.notes.clone(),
            };
            BrandUpdateTemplate {
                brand: Some(b),
                form,
                errors: None,
                current_page: "brand".to_string(),
                error_message: None,
                success_message: None
            }.into_response()
        }
        Ok(None) => Redirect::to("/web/brands?error=not_found").into_response(),
        Err(_) => Redirect::to("/web/brands?error=db_error").into_response(),
    }
}




pub async fn edit_brand(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Form(form): Form<CreateBrandForm>,
) -> Response {
    // 1. التحقق من صحة المدخلات
    if let Err(form_err) = form.validate() {
        return BrandUpdateTemplate {
            brand: fetch_brand_by_id(&state, id).await,
            form,
            errors: Some(form_err),
            current_page: "brand".to_string(),
            error_message: Some("يرجى تصحيح الأخطاء لإستكمال التعديل".to_string()),
            success_message: None,
        }
        .into_response();
    }

    let trimmed_name_en = form.name_en.trim();
    let trimmed_name_ar = form.name_ar.trim();
    let trimmed_notes = form
        .notes
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from);

    // 2. جلب السجل الحالي من قاعدة البيانات
    let existing_brand = match fetch_brand_by_id(&state, id).await {
        Some(brand) => brand,
        None => return Redirect::to("/web/brands?error=not_found").into_response(),
    };

    // 3. التحقق مما إذا كانت البيانات مطابقة تماماً دون أي تغيير
    let is_unchanged = existing_brand.name_en == trimmed_name_en
        && existing_brand.name_ar == trimmed_name_ar
        && existing_brand.notes == trimmed_notes;

    if is_unchanged {
        return BrandUpdateTemplate {
            brand: Some(existing_brand),
            form,
            errors: None,
            current_page: "brand".to_string(),
            error_message: Some("لم يتم إجراء أي تغييرات على البيانات".to_string()),
            success_message: None,
        }
        .into_response();
    }

    // 4. تنفيذ الاستعلام فقط في حال وجود تغيير حقيقي
    let result = sqlx::query!(
        r#"UPDATE brands SET name_en = $1, name_ar = $2, notes = $3 WHERE id = $4"#,
        trimmed_name_en,
        trimmed_name_ar,
        trimmed_notes,
        id
    )
    .execute(&state.pool)
    .await;

    // 5. معالجة النتيجة
    match result {
        Ok(_) => Redirect::to("/web/brands?action=updated").into_response(),

        Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23505") => {
            BrandUpdateTemplate {
                brand: Some(existing_brand),
                form,
                errors: None,
                current_page: "brand".to_string(),
                error_message: Some("هذا البيان مسجل بالفعل".to_string()),
                success_message: None,
            }
            .into_response()
        }

        Err(_) => BrandUpdateTemplate {
            brand: Some(existing_brand),
            form,
            errors: None,
            current_page: "brand".to_string(),
            error_message: Some("حدث خطأ عام .. برجاء المحاولة لاحقاً".to_string()),
            success_message: None,
        }
        .into_response(),
    }
}
