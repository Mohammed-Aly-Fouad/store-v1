use std::{collections::HashMap, fmt::Display, str::FromStr};

use askama::Template;
use askama_web::WebTemplate;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use sqlx::prelude::FromRow;



pub fn empty_number_as_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: Display,
{
    let opt = Option::<String>::deserialize(deserializer)?;

    match opt.as_deref().map(str::trim) {
        Some("") | None => Ok(None),
        Some(s) => s.parse::<T>().map(Some).map_err(serde::de::Error::custom),
    }
}

pub fn empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    Ok(opt.filter(|s| !s.trim().is_empty()))
}

// 1. DTO --> Askama
#[derive(Deserialize, Serialize, FromRow, Clone, Debug)]
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
// 2. Template
#[derive(Template, WebTemplate)]
#[template(path = "categories/index.html")]
pub struct CategoryTemplate {
    pub main_categories: Vec<CategoryResponseDTO>,
    pub sub_categories: Vec<CategoryResponseDTO>,
    pub error_message: Option<String>,
    pub success_message: Option<String>,
    pub current_page: String,

}
// Helper struct for Asakmak
#[derive(Default, Debug, Serialize)]
pub struct CategoryFormErrors {
    pub name_en: Option<String>,
    pub name_ar: Option<String>,
    pub parent_id: Option<i64>,
    pub parent_name: Option<String>,
    pub notes: Option<String>,
}

impl CategoryFormErrors {
    pub fn has_errors(&self) -> bool {
        self.name_en.is_some() || self.name_ar.is_some() || self.notes.is_some()
    }
}

// 1. Struct of create page form
#[derive(Debug, Deserialize, Default)]
pub struct CreateCategoryForm {
    pub name_en: String,
    pub name_ar: String,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub parent_name: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub notes: Option<String>,
}

impl CreateCategoryForm {
    pub fn validate(&self) -> Result<(), CategoryFormErrors> {
        let mut errors = CategoryFormErrors::default();

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

         // فحص parent_name
        if let Some(parent_name) = &self.parent_name {
            let trimmed_parent_name = parent_name.trim();
            if trimmed_parent_name.chars().count() > 500 {
                errors.parent_name = Some("لا يمكن أن يزيد عدد أحرف الفئة عن 500 حرف".to_string());
            }
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

// 2. Tempplate for create page

#[derive(Template, WebTemplate)]
#[template(path = "categories/create.html")]
pub struct CategoryCreateTemplate {
    pub main_categories: Vec<CategoryResponseDTO>,
    pub sub_categories: Vec<CategoryResponseDTO>,
    pub form: CreateCategoryForm,
    pub errors: Option<CategoryFormErrors>,
    pub current_page: String,
    pub error_message: Option<String>,
    pub success_message: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CategoryRow {
    pub id: i64,
    pub name: String,
    pub name_ar: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub parent_name: Option<String>,
}

impl CategoryRow {
    pub fn build_rows(categories: &[CategoryResponseDTO]) -> Vec<CategoryRow> {
        let id_to_name: HashMap<i64, &str> =
            categories.iter().map(|c| (c.id, c.name_en.as_str())).collect();

        categories
            .iter()
            .map(|c| CategoryRow {
                id: c.id,
                name: c.name_en.clone(),
                name_ar: c.name_ar.clone(),
                notes: c.notes.clone(),
                created_at: c.created_at,
                parent_name: c
                    .parent_id
                    .and_then(|pid| id_to_name.get(&pid).map(|n| n.to_string())),
            })
            .collect()
    }
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
