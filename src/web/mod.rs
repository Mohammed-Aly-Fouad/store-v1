pub mod brand;
pub mod category;
pub mod category2;


use crate::{state::AppState};
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/brands", brand::router())
        .nest("/categories", category::router())
        .nest("/test", category2::router())

      
}