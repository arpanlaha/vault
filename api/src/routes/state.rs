use super::utils::State;
use warp::{Filter, Rejection, Reply};

pub use handlers::LastUpdated;

/// Wraps all `Graph` state routes.
#[must_use]
pub fn routes(state: State) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    time_since_last_update(state)
}

/// Returns the time (in seconds) since the `Graph` was last updated.
fn time_since_last_update(
    state: State,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("state" / "last-updated")
        .and(warp::get())
        .and_then(move || handlers::time_since_last_update(state.clone()))
}

mod handlers {
    use super::State;
    use serde::{Deserialize, Serialize};
    use warp::{reply, Rejection, Reply};

    /// Returns the time (in seconds) since the `Graph` was last updated.
    pub async fn time_since_last_update(state: State) -> Result<impl Reply, Rejection> {
        Ok(reply::json(&LastUpdated {
            seconds: state.time_since_last_update(),
        }))
    }

    /// A struct containing the time since the `Graph` was last updated.
    #[derive(Deserialize, Serialize)]
    pub struct LastUpdated {
        /// The time (in seconds) since the `Graph` was last updated.
        pub seconds: u64,
    }
}
