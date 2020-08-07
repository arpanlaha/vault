use parking_lot::RwLock;
use std::convert::Infallible;
use std::sync::Arc;
use vault_graph::Graph;
use warp::{http::StatusCode, reject::Reject, Rejection, Reply};

/// Shorthand for Arc<RwLock<Graph>>.
pub type State = Arc<RwLock<Graph>>;

#[derive(Debug)]
pub enum VaultError {
    CategoryNotFound(String),
    CrateNotFound(String),
    KeywordNotFound(String),
    UpdateForbidden,
}

impl Reject for VaultError {}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, String::from("Route not found."))
    } else if let Some(e) = err.find::<VaultError>() {
        match e {
            VaultError::CategoryNotFound(category_id) => (
                StatusCode::NOT_FOUND,
                format!("Category with id {} not found.", category_id),
            ),
            VaultError::CrateNotFound(crate_id) => (
                StatusCode::NOT_FOUND,
                format!("Crate with id {} not found.", crate_id),
            ),
            VaultError::KeywordNotFound(keyword_id) => (
                StatusCode::NOT_FOUND,
                format!("Keyword with id {} not found.", keyword_id),
            ),
            VaultError::UpdateForbidden => (
                StatusCode::FORBIDDEN,
                String::from("Updating application state can only occur in 24-hour intervals."),
            ),
        }
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        (
            StatusCode::METHOD_NOT_ALLOWED,
            String::from("Method Not Allowed"),
        )
    } else {
        eprintln!("unhandled error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Internal Server Error"),
        )
    };

    Ok(warp::reply::with_status(warp::reply::json(&message), code))
}
