use super::utils::{State, VaultError};
use warp::{Filter, Rejection, Reply};

pub fn routes(state: State) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    get_keyword(state.clone())
        .or(random(state.clone()))
        .or(search(state))
}

fn get_keyword(state: State) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("keywords" / String)
        .and(warp::get())
        .and_then(move |keyword_id| handlers::get_keyword(keyword_id, state.clone()))
}

fn random(state: State) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("random" / "keywords")
        .and(warp::get())
        .and_then(move || handlers::random(state.clone()))
}

fn search(state: State) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("search" / "keywords" / String)
        .and(warp::get())
        .and_then(move |search_term| handlers::search(search_term, state.clone()))
}

mod handlers {
    use super::{State, VaultError};
    use vault_graph::{Random, Search};
    use warp::{reject, reply, Rejection, Reply};

    /// Returns the `Keyword` with the given id, if found.
    ///
    /// # Errors
    /// * Returns a `404` error if no `Keyword` with the given id is found.
    pub async fn get_keyword(keyword_id: String, state: State) -> Result<impl Reply, Rejection> {
        match state.read().keywords().get(&keyword_id) {
            None => Err(reject::custom(VaultError::KeywordNotFound(keyword_id))),

            Some(keyword) => Ok(reply::json(keyword)),
        }
    }

    /// Returns a random `Keyword`.
    pub async fn random(state: State) -> Result<impl Reply, Rejection> {
        Ok(reply::json(state.read().keywords().random()))
    }

    /// Searches for keywords matching the given search term.
    pub async fn search(search_term: String, state: State) -> Result<impl Reply, Rejection> {
        let graph = state.read();

        Ok(reply::json(
            &graph.keyword_names().search(&search_term, graph.keywords()),
        ))
    }
}
