use askama::Template;
use askama_web::WebTemplate;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::FromRow;

// Unused imports cleared (e.g., crate::domain::brand::model::Brand)

//


/// Shared Data Transfer Object representing a single Brand entity.
///
/// Used seamlessly across both standard JSON REST APIs and HTML Askama templates.

#[derive(Debug, Serialize, FromRow, Clone)]

pub struct BrandResponseDTO {
    pub id: i64,
    pub name_en: String,
    pub name_ar: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}