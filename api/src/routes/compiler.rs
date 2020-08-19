use super::utils::State;
use warp::{Filter, Rejection, Reply};

pub use handlers::{CfgNameList, TargetList};

/// Wraps all compiler-related routes.
pub fn routes(state: State) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    get_targets(state.clone()).or(get_cfg_names(state))
}

/// Returns a list of targets.
fn get_targets(state: State) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("compiler" / "targets")
        .and(warp::get())
        .and_then(move || handlers::get_targets(state.clone()))
}

/// Returns a list of cfg names.
fn get_cfg_names(state: State) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("compiler" / "cfg_names")
        .and(warp::get())
        .and_then(move || handlers::get_cfg_names(state.clone()))
}

mod handlers {
    use super::State;
    use serde::Serialize;
    use warp::{reply, Rejection, Reply};

    /// Returns a list of targets.
    pub async fn get_targets(state: State) -> Result<impl Reply, Rejection> {
        Ok(reply::json(&TargetList {
            targets: state.read().targets().keys().collect(),
        }))
    }

    /// Returns a list of cfg names.
    pub async fn get_cfg_names(state: State) -> Result<impl Reply, Rejection> {
        Ok(reply::json(&CfgNameList {
            cfg_names: state.read().cfg_names().iter().collect(),
        }))
    }

    /// A struct for sending a response containing a list of targets.
    #[derive(Serialize)]
    pub struct TargetList<'a> {
        pub targets: Vec<&'a String>,
    }

    /// A struct for sending a response containing a list of cfg names.
    #[derive(Serialize)]
    pub struct CfgNameList<'a> {
        pub cfg_names: Vec<&'a String>,
    }
}
