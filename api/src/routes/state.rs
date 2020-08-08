use super::utils::{State, VaultError};
use warp::{Filter, Rejection, Reply};

pub use handlers::LastUpdated;

pub fn routes(state: State) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    time_since_last_update(state.clone()).or(reset(state))
}

fn time_since_last_update(
    state: State,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("state" / "last-updated")
        .and(warp::get())
        .and_then(move || handlers::time_since_last_update(state.clone()))
}

fn reset(state: State) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("state" / "reset")
        .and(warp::put())
        .and_then(move || handlers::reset(state.clone()))
}

mod handlers {
    use super::{State, VaultError};
    use serde::{Deserialize, Serialize};
    use vault_graph::Graph;
    use warp::{reject, reply, Rejection, Reply};

    /// The minimum interval of time before a state update is permitted.
    ///
    /// The crates.io database dump is updated daily, so this interval lies just under a day to permit some leeway.
    const INTERVAL: u64 = 60 * (60 * 23 + 55);

    /// Returns a list of all categories.
    pub async fn time_since_last_update(state: State) -> Result<impl Reply, Rejection> {
        Ok(reply::json(&LastUpdated {
            seconds: state.read().time_since_last_update(),
        }))
    }

    /// A helper method to determine if the `Graph` can be updated.
    ///
    /// # Arguments
    /// * `state` - the app data containing the `Graph`.
    fn can_update(state: &State) -> bool {
        let mut graph = state.write();
        if graph.time_since_last_update() >= INTERVAL {
            graph.update_time();
            true
        } else {
            false
        }
    }

    /// A helper method to determine if the `Graph` can be updated.
    ///
    /// # Arguments
    /// * `state` - the app data containing the `Graph`.
    pub async fn reset(state: State) -> Result<impl Reply, Rejection> {
        if can_update(&state) {
            let new_graph = Graph::new().await;
            state.write().replace(new_graph);

            Ok(reply::json(&"Successfully updated application state."))
        } else {
            Err(reject::custom(VaultError::UpdateForbidden))
        }
    }

    /// A struct containing the time since the `Graph` was last updated.
    #[derive(Deserialize, Serialize)]
    pub struct LastUpdated {
        /// The time (in seconds) since the `Graph` was last updated.
        pub seconds: u64,
    }
}
