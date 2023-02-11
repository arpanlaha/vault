#![allow(clippy::missing_errors_doc)]

use std::{convert::Infallible, sync::Arc};
use vault_graph::Graph;
use warp::{http::StatusCode, reject::Reject, Rejection, Reply};

/// Shorthand for Arc<Graph>.
pub type State = Arc<Graph>;

/// An enum corresponding to custom errors which may occur.
#[derive(Debug)]
pub enum VaultError {
    /// If the provided `Category` does not exist.
    CategoryNotFound(String),

    /// If the provided `Crate` does not exist.
    CrateNotFound(String),

    /// If the provided `Keyword` does not exist.
    KeywordNotFound(String),

    /// If options passed in query parameters do not exist.
    NonexistentOptions(Vec<String>),
}

impl Reject for VaultError {}

/// A `warp` function to handle different errors.
///
/// # Arguments
/// * `err` - the `Rejection` to handle.
pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, String::from("Route not found."))
    } else if let Some(e) = err.find::<VaultError>() {
        match e {
            VaultError::CategoryNotFound(category_id) => (
                StatusCode::NOT_FOUND,
                format!("Category with id {category_id} not found."),
            ),

            VaultError::CrateNotFound(crate_id) => (
                StatusCode::NOT_FOUND,
                format!("Crate with id {crate_id} not found."),
            ),

            VaultError::KeywordNotFound(keyword_id) => (
                StatusCode::NOT_FOUND,
                format!("Keyword with id {keyword_id} not found."),
            ),

            VaultError::NonexistentOptions(nonexistent_options) => (
                StatusCode::BAD_REQUEST,
                format!(
                    "The following options were provided with improper values: {},",
                    nonexistent_options.join(", ")
                ),
            ),
        }
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        (
            StatusCode::METHOD_NOT_ALLOWED,
            String::from("Method Not Allowed"),
        )
    } else {
        eprintln!("unhandled error: {err:?}");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Internal Server Error"),
        )
    };

    Ok(warp::reply::with_status(warp::reply::json(&message), code))
}
