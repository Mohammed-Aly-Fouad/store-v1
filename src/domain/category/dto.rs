use askama::Template;
use askama_web::WebTemplate;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Deserialize, Serialize, FromRow, Clone)]
pub struct CategoryResponseDTO {
    pub id: i64,
    pub name_en: String,
    pub name_ar: String,
    pub parent_id: Option<i64>,
    pub parent_name: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Template, WebTemplate)]
#[template(path = "categories/index.html")]
pub struct CategoryTemplate {
    pub categories: Vec<CategoryResponseDTO>,
    pub error_message: Option<String>,
    pub success_message: Option<String>,
    pub current_page: String,

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
