use super::utils::{State, VaultError};
use std::collections::HashMap;
use warp::{Filter, Rejection, Reply};

/// Wraps all `Crate` routes.
pub fn routes(state: State) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    get_crate(state.clone())
        .or(random(state.clone()))
        .or(search(state.clone()))
        .or(get_dependency_graph(state.clone()))
        .or(get_random_dependency_graph(state))
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
                query_param_map
                    .get("target")
                    .map(|query_param| query_param.to_owned()),
                query_param_map
                    .get("cfg_name")
                    .map(|query_param| query_param.to_owned()),
                state.clone(),
            )
        })
}

/// Returns the `DependencyGraph` of a random `Crate`.
fn get_random_dependency_graph(
    state: State,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("random" / "graph")
        .and(warp::get())
        .and_then(move || handlers::get_random_dependency_graph(state.clone()))
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
    /// * Returns a `400` error if the `platform` or `cfg_name` query options reference nonexistent values.
    pub async fn get_dependency_graph(
        crate_id: String,
        features_option: Option<String>,
        target_option: Option<String>,
        cfg_name_option: Option<String>,
        state: State,
    ) -> Result<impl Reply, Rejection> {
        let state = state.read();

        let mut nonexistent_options: Vec<String> = vec![];

        // check if target was provided and exists
        if let Some(target) = target_option.clone() {
            if !state.targets().contains_key(&target) {
                nonexistent_options.push(String::from("target"));
            }
        }

        // check if cfg name was provided and exists
        if let Some(cfg_name) = cfg_name_option.clone() {
            if !state.cfg_names().contains(&cfg_name) {
                nonexistent_options.push(String::from("cfg_name"));
            }
        }

        if nonexistent_options.is_empty() {
            match &state.get_dependency_graph(
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
                &target_option,
                &cfg_name_option,
            ) {
                None => Err(reject::custom(VaultError::CrateNotFound(crate_id))),

                Some(dependency_graph) => Ok(reply::json(dependency_graph)),
            }
        } else {
            Err(reject::custom(VaultError::NonexistentOptions(
                nonexistent_options,
            )))
        }
    }

    /// Returns the `DependencyGraph` of a random `Crate`.
    pub async fn get_random_dependency_graph(state: State) -> Result<impl Reply, Rejection> {
        let state = state.read();

        Ok(reply::json(
            &state
                .get_dependency_graph(&state.crates().random().name, vec![], &None, &None)
                .unwrap(),
        ))
    }
}
