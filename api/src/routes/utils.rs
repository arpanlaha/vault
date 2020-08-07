use parking_lot::RwLock;
use std::sync::Arc;
use vault_graph::Graph;
use warp::reject::Reject;

/// Shorthand for Arc<RwLock<Graph>>.
pub type State = Arc<RwLock<Graph>>;

#[derive(Debug)]
pub enum VaultError {
    UpdateForbidden,
    IdNotFound(String, String),
}

impl Reject for VaultError {}
