use askama::Template;
use askama_web::WebTemplate;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::FromRow;


#[derive(Debug, Serialize, FromRow, Clone, Deserialize)]
pub struct BrandResponseDTO {
    pub id: i64,
    pub name_en: String,
    pub name_ar: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Default)]
pub struct CreateBrandForm {
    pub name_en: String,
    pub name_ar: String,
    pub notes: Option<String>,
}

impl CreateBrandForm {
    pub fn validate(&self) -> Result<(), BrandFormErrors> {
        let mut errors = BrandFormErrors::default();

        // فحص name_en
        let trimmed_name_en = self.name_en.trim();
        if trimmed_name_en.is_empty() {
            errors.name_en = Some("لا يمكن ترك هذه القيمة فارغة".to_string());
        } else if trimmed_name_en.chars().count() > 100 {
            errors.name_en = Some("لا يمكن أن يزيد عدد أحرف هذه الخانة عن 100 حرف".to_string());
        }

        // فحص name_ar
        let trimmed_name_ar = self.name_ar.trim();
        if trimmed_name_ar.is_empty() {
            errors.name_ar = Some("لا يمكن ترك هذه القيمة فارغة".to_string());
        } else if trimmed_name_ar.chars().count() > 100 {
            errors.name_ar = Some("لا يمكن أن يزيد عدد أحرف هذه الخانة عن 100 حرف".to_string());
        }

        // فحص notes
        if let Some(notes) = &self.notes {
            let trimmed_notes = notes.trim();
            if trimmed_notes.chars().count() > 500 {
                errors.notes = Some("لا يمكن أن يزيد عدد أحرف الملاحظات عن 500 حرف".to_string());
            }
        }

        if errors.has_errors() {
            Err(errors)
        } else {
            Ok(())
        }
    }
}

#[derive(Default, Debug, Serialize)]
pub struct BrandFormErrors {
    pub name_en: Option<String>,
    pub name_ar: Option<String>,
    pub notes: Option<String>,
}

impl BrandFormErrors {
    pub fn has_errors(&self) -> bool {
        self.name_en.is_some() || self.name_ar.is_some() || self.notes.is_some()
    }
}





// ============================================================================
// SECTION 4: ASKAMA TEMPLATES & UI FILTERS
// ============================================================================

/// Main page Askama template rendering brand management dashboard (`brands.html`).
#[derive(Template, WebTemplate)]
#[template(path = "brands/index.html")]
pub struct BrandsTemplate {
    pub brands: Vec<BrandResponseDTO>,
    // pub brand_form_errors: Option<BrandFormErrors>,
    pub error_message: Option<String>,
    pub success_message: Option<String>,
    // pub edit_brand: Option<BrandResponseDTO>,
    pub current_page: String,
    // pub form_data: Option<CreateBrandForm>,
    // pub show_modal: bool
    
}


#[derive(Template, WebTemplate)]
#[template(path = "brands/create.html")]
pub struct BrandCreateTemplate {
    pub form: CreateBrandForm,
    pub errors: Option<BrandFormErrors>,
    pub current_page: String,
    pub error_message: Option<String>,
    pub success_message: Option<String>,
}


#[derive(Template, WebTemplate)]
#[template(path = "brands/edit.html")]
pub struct BrandUpdateTemplate {
    pub brand: Option<BrandResponseDTO>,
    pub form: CreateBrandForm,
    pub errors: Option<BrandFormErrors>,
    pub current_page: String,
    pub error_message: Option<String>,
    pub success_message: Option<String>,
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

// We name it filter so no need to "use filters"
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



// ============================================================================
// FLASH MESSAGES & QUERY PARAMS
// ============================================================================

/// Query parameters used to carry one-time "Flash Messages" across HTTP Redirects.

#[derive(Debug, Deserialize)]
pub struct FlashParams {
    pub action: Option<String>,
    pub error: Option<String>
}




