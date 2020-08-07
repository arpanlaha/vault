use parking_lot::RwLock;
use std::sync::Arc;
use vault_graph::Graph;
use warp::reject::Reject;

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
