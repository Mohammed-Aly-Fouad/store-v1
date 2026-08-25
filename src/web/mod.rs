pub mod brand;
pub mod category;


use crate::{state::AppState};
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/brands", brand::router())
        .nest("/categories", category::router())
      
}