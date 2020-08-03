use actix_web::web::Data;
use std::collections::HashMap;
use tokio::sync::RwLock;
use vault_graph::Graph;

/// Shorthand for Data<RwLock<Graph>>.
pub type State = Data<RwLock<Graph>>;

/// An enum containing possible query param errors.
pub enum QueryParamError {
    /// If the query string (or any query parameter) does not contain the `=` character.
    InvalidQueryString,
}

/// Maps a query string into a `HashMap` of key-value pairs.
pub fn get_query_params(query_str: &str) -> Result<HashMap<String, String>, QueryParamError> {
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
