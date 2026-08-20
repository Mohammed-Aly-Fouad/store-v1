use askama::Template;
use askama_web::WebTemplate;
use axum::{
    extract::{Form, Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
    Router,
};
use serde::Deserialize;

use crate::domain::brand::dto::BrandResponseDTO;

use crate::state::AppState;

// ============================================================================
// ROUTER CONFIGURATION
// ============================================================================

/// Configures and returns the sub-router for all browser-based Askama HTML endpoints.
pub fn router() -> Router<AppState> {
    Router::new()
    .route("/", get(render_brands_page))

    }


// ============================================================================
// FLASH MESSAGES & QUERY PARAMS
// ============================================================================

/// Query parameters used to carry one-time "Flash Messages" across HTTP Redirects.
///
/// **لماذا نستخدم هذا النمط؟**
/// بعد أي عملية نجاح (إنشاء / تعديل / حذف)، نقوم بعمل `Redirect` لمنع تكرار الإرسال
/// عند عمل (Refresh). وبما أن الـ Redirect ينشئ طلب `GET` جديد تماماً، تفقد الاستجابة
/// بيانات الـ Context السابقة. نمرر إشارة مثل (`?ok=created`) ونقرأها في صفحة العرض.
#[derive(Debug, Deserialize)]
pub struct FlashParams {
    pub ok: Option<String>,
}

// ============================================================================
// DATABASE HELPER FUNCTIONS
// ============================================================================

/// يجلب كافة البراندات مرتبة تنازلياً حسب المعرف (`ID`).
/// 
/// في حالة حدوث خطأ في قاعدة البيانات، تُرجع الدالة قائمة فارغة لضمان
/// استمرارية عمل واجهة المستخدم وعدم توقف الصفحة بالكامل.
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
// HANDLERS: READ & RENDER
// ============================================================================

/// Renders the main HTML page containing the brand list and creation form.
///
/// Checks for flash params (`?ok=...`) in the query string to display contextual 
/// success notifications after a redirect.
pub async fn render_brands_page(
    State(state): State<AppState>,
    Query(params): Query<FlashParams>,
) -> BrandsTemplate {
    let success_message = match params.ok.as_deref() {
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
        edit_brand: None,
    }
}

/// Renders the main page with a specific brand pre-loaded for editing inside a modal or inline form.
pub async fn edit_brand_page(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> BrandsTemplate {
    BrandsTemplate {
        brands: fetch_all_brands(&state).await,
        error_message: None,
        success_message: None,
        current_page: "brands".to_string(),
        edit_brand: fetch_brand_by_id(&state, id).await,
    }
}


// ============================================================================
// SECTION 4: ASKAMA TEMPLATES & UI FILTERS
// ============================================================================

/// Main page Askama template rendering brand management dashboard (`brands.html`).
#[derive(Template, WebTemplate)]
#[template(path = "brands.html")]
pub struct BrandsTemplate {
    pub brands: Vec<BrandResponseDTO>,
    pub error_message: Option<String>,
    pub success_message: Option<String>,
    pub edit_brand: Option<BrandResponseDTO>,
    pub current_page: String,
}

/// Partial HTML snippet template for HTMX/Dynamic live brand search.
#[derive(Template, WebTemplate)]
#[template(path = "partials/brand_search_results.html")]
pub struct BrandSearchResultsTemplate {
    pub brands: Vec<BrandResponseDTO>,
    pub query: String,
}

/// URL Query parameter extractor for brand live-search requests.
#[derive(Debug, Deserialize)]
pub struct BrandSearchQuery {
    #[serde(default)]
    pub q: String,
}

// ---------------------------------------------------------------------------
// 4.1 Custom Askama Filters
// ---------------------------------------------------------------------------

pub mod filters {
    use askama::Values;

    /// Extract the capitalized initial character of a brand name for UI avatar circles.
    #[askama::filter_fn]
    pub fn first_letter(name: &str, _values: &dyn Values) -> askama::Result<String> {
        Ok(name
            .trim()
            .chars()
            .next()
            .map(|c| c.to_uppercase().to_string())
            .unwrap_or_else(|| "؟".to_string()))
    }

    /// Generates a deterministic hex color code based on the brand string hash.
    /// Ensures the same brand always gets the exact same avatar background color across renders.
    #[askama::filter_fn]
    pub fn initial_color(name: &str, _values: &dyn Values) -> askama::Result<String> {
        const PALETTE: [&str; 6] = [
            "#0E7C66", "#2563EB", "#D97706", "#7C3AED", "#DB2777", "#0891B2",
        ];
        let sum: u32 = name.bytes().map(|b| b as u32).sum();
        Ok(PALETTE[sum as usize % PALETTE.len()].to_string())
    }
}