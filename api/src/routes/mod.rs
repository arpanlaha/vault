mod categories;
mod crates;
mod keywords;
mod state;
use warp::{Filter, Rejection, Reply};

use super::utils::State;

pub fn get(state: State) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    categories::routes(state.clone())
        .or(crates::routes(state.clone()))
        .or(keywords::routes(state.clone()))
        .or(state::routes(state.clone()))
}
