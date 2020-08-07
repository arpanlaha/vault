use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use vault_graph::Graph;
use warp::{reject::Reject, Filter};

/// Shorthand for Arc<RwLock<Graph>>.
pub type State = Arc<RwLock<Graph>>;

pub fn with_state(
    state: State,
) -> impl Filter<Extract = (State,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}

#[derive(Debug)]
pub enum VaultError {
    QueryParamError,
    UpdateForbidden,
    IdNotFound(String, String),
}

impl Reject for VaultError {}
/// An enum containing possible query param errors.
pub enum QueryParamError {
    /// If the query string (or any query parameter) does not contain the `=` character.
    InvalidQueryString,
}

/// Maps a query string into a `HashMap` of key-value pairs.
pub fn get_query_params(query_str: String) -> Result<HashMap<String, String>, QueryParamError> {
    if query_str.is_empty() {
        return Ok(HashMap::new());
    }

    if query_str.contains('&') {
        let mut query_param_strs = query_str.split('&');

        if query_param_strs.any(|query_param_str| !query_param_str.contains('=')) {
            return Err(QueryParamError::InvalidQueryString);
        }

        Ok(query_param_strs
            .map(|query_param_str| {
                let mut query_param_split = query_param_str.split('=');
                (
                    String::from(query_param_split.next().unwrap()),
                    String::from(query_param_split.next().unwrap()),
                )
            })
            .collect::<HashMap<String, String>>())
    } else {
        if !query_str.contains('=') {
            return Err(QueryParamError::InvalidQueryString);
        }
        let mut query_param_split = query_str.split('=');
        let mut query_param_map: HashMap<String, String> = HashMap::new();
        query_param_map.insert(
            String::from(query_param_split.next().unwrap()),
            String::from(query_param_split.next().unwrap()),
        );

        Ok(query_param_map)
    }
}
