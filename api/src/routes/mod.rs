pub mod categories;
pub mod compiler;
pub mod crates;
pub mod keywords;
pub mod state;
pub mod utils;

use utils::State;
use warp::{Filter, Rejection, Reply};

/// Wraps all routes.
pub fn get(state: State) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    crates::routes(state.clone())
        .or(compiler::routes(state.clone()))
        .or(state::routes(state.clone()))
        .or(keywords::routes(state.clone()))
        .or(categories::routes(state))
}
