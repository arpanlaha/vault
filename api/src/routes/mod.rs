pub mod categories;
pub mod crates;
pub mod keywords;
pub mod state;
pub mod utils;

use warp::{Filter, Rejection, Reply};

use utils::State;

pub fn get(state: State) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    categories::routes(state.clone())
        .or(crates::routes(state.clone()))
        .or(keywords::routes(state.clone()))
        .or(state::routes(state))
}
