use super::utils::{State, VaultError};
use std::collections::HashMap;
use warp::{Filter, Rejection, Reply};

/// Wraps all `Crate` routes.
pub fn routes(state: State) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    get_crate(state.clone())
        .or(random(state.clone()))
        .or(search(state.clone()))
        .or(get_dependency_graph(state))
}

/// Returns the `Crate` with the given id, if found.
///
/// # Errors
/// * Returns a `404` error if no `Crate` with the given id is found.
fn get_crate(state: State) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("crates" / String)
        .and(warp::get())
        .and_then(move |crate_id| handlers::get_crate(crate_id, state.clone()))
}

/// Returns a random `Crate`.
fn random(state: State) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("random" / "crates")
        .and(warp::get())
        .and_then(move || handlers::random(state.clone()))
}

/// Searches for crates matching the given search term.
fn search(state: State) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("search" / "crates" / String)
        .and(warp::get())
        .and_then(move |search_term| handlers::search(search_term, state.clone()))
}

/// Returns the `DependencyGraph` of the `Crate` ith the given id, if found.
///
/// # Errors
/// * Returns a `404` error if no `Crate` with the given id is found.
fn get_dependency_graph(
    state: State,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("graph" / String)
        .and(warp::get())
        .and(warp::query::<HashMap<String, String>>())
        .and_then(move |crate_id, query_param_map: HashMap<String, String>| {
            handlers::get_dependency_graph(
                crate_id,
                query_param_map
                    .get("features")
                    .map(|query_param| query_param.to_owned()),
                state.clone(),
            )
        })
}

mod handlers {
    use super::{State, VaultError};
    use vault_graph::{Random, Search};
    use warp::{reject, reply, Rejection, Reply};

    /// Returns the `Crate` with the given id, if found.
    ///
    /// # Errors
    /// * Returns a `404` error if no `Crate` with the given id is found.
    pub async fn get_crate(crate_id: String, state: State) -> Result<impl Reply, Rejection> {
        match state.read().crates().get(&crate_id) {
            None => Err(reject::custom(VaultError::CrateNotFound(crate_id))),

            Some(crate_val) => Ok(reply::json(crate_val)),
        }
    }

    /// Returns a random `Crate`.
    pub async fn random(state: State) -> Result<impl Reply, Rejection> {
        Ok(reply::json(state.read().crates().random()))
    }

    /// Searches for crates matching the given search term.
    pub async fn search(search_term: String, state: State) -> Result<impl Reply, Rejection> {
        let graph = state.read();

        Ok(reply::json(
            &graph.crate_names().search(&search_term, graph.crates()),
        ))
    }

    /// Returns the `DependencyGraph` of the `Crate` ith the given id, if found.
    ///
    /// # Errors
    /// * Returns a `404` error if no `Crate` with the given id is found.
    pub async fn get_dependency_graph(
        crate_id: String,
        features_option: Option<String>,
        state: State,
    ) -> Result<impl Reply, Rejection> {
        match &state.read().get_dependency_graph(
            &crate_id,
            match features_option {
                Some(features) => {
                    if features.contains(',') {
                        features
                            .split(',')
                            .map(String::from)
                            .collect::<Vec<String>>()
                    } else {
                        vec![features]
                    }
                }
                None => vec![],
            },
            "x86_64-unknown-linux-gnu"
        ) {
            None => Err(reject::custom(VaultError::CrateNotFound(crate_id))),

            Some(dependency_graph) => Ok(reply::json(dependency_graph)),
        }
    }
}
