use std::sync::Arc;
use vault_api::routes::utils::State;
use vault_graph::Graph;

/// Returns a new test instance of `State`.
pub fn get_data() -> State {
    Arc::new(Graph::test())
}
