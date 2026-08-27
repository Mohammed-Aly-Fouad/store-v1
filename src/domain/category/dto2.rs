use askama::Template;
use askama_web::WebTemplate;
use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use serde::Deserializer;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;


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

// ####################################

#[derive(Deserialize, Serialize, FromRow, Clone, Debug)]
pub struct CategoryResponseDto {
    pub id: i64,
    pub name_en: String,
    pub name_ar: String,
    pub parent_id: Option<i64>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Clone)]
pub struct CategoryTreeDto {
    pub id: i64,
    pub name_en: String,
    pub name_ar: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub children: Vec<CategoryTreeDto>,
}

impl CategoryTreeDto {
    fn from_flat(cat: &CategoryResponseDto) -> Self {
        CategoryTreeDto {
            id: cat.id,
            name_en: cat.name_en.clone(),
            name_ar: cat.name_ar.clone(),
            notes: cat.notes.clone(),
            created_at: cat.created_at,
            updated_at: cat.updated_at,
            children: Vec::new(),
        }
    }

    pub fn build_tree(flat_categories: Vec<CategoryResponseDto>) -> Vec<CategoryTreeDto> {
        let mut children_map: HashMap<i64, Vec<&CategoryResponseDto>> = HashMap::new();
        let mut roots: Vec<&CategoryResponseDto> = Vec::new();

        for cat in &flat_categories {
            match cat.parent_id {
                Some(pid) => children_map.entry(pid).or_default().push(cat),
                None => roots.push(cat),
            }
        }

        roots
            .into_iter()
            .map(|root| Self::build_node(root, &children_map))
            .collect()
    }

    fn build_node(
        cat: &CategoryResponseDto,
        children_map: &HashMap<i64, Vec<&CategoryResponseDto>>,
    ) -> CategoryTreeDto {
        let mut node = Self::from_flat(cat);

        if let Some(children) = children_map.get(&cat.id) {
            node.children = children
                .iter()
                .map(|child| Self::build_node(child, children_map))
                .collect();
        }

        node
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateCategoryForm {
    pub name: String,
    pub name_ar: String,

    // Uses i32 helper
    #[serde(default, deserialize_with = "empty_number_as_none")]
    pub parent_id: Option<i64>,

    // Uses String helper
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub notes: Option<String>,
}


#[derive(Debug, Clone)]
pub struct CategoryRow {
    pub id: i64,
    pub name_en: String,
    pub name_ar: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub parent_name: Option<String>,
}

impl CategoryRow {
    pub fn build_rows(categories: &[CategoryResponseDto]) -> Vec<CategoryRow> {
        let id_to_name: HashMap<i64, &str> =
            categories.iter().map(|c| (c.id, c.name_en.as_str())).collect();

        categories
            .iter()
            .map(|c| CategoryRow {
                id: c.id,
                name_en: c.name_en.clone(),
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
#[derive(Template, WebTemplate)]
#[template(path = "categories.html")]
pub struct CategoryTemplate {
    pub categories: Vec<CategoryRow>,
    pub root_categories: Vec<CategoryResponseDto>,
    pub error_message: Option<String>,
    pub success_message: Option<String>,
    pub current_page: String,

}

