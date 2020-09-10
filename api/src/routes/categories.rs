use super::utils::{State, VaultError};
use warp::{Filter, Rejection, Reply};

pub use handlers::CategoryResponse;

/// Wraps all `Category` routes.
#[must_use]
pub fn routes(state: State) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    get_categories(state.clone())
        .or(get_category(state.clone()))
        .or(random(state.clone()))
        .or(search(state))
}

/// Returns a list of all categories.
fn get_categories(state: State) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("categories")
        .and(warp::path::end())
        .and(warp::get())
        .and_then(move || handlers::get_categories(state.clone()))
}

/// Returns the `Category` with the given id, if found.
///
/// # Errors
/// * Returns a `404` error if no `Category` with the given id is found.
fn get_category(state: State) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("categories" / String)
        .and(warp::get())
        .and_then(move |category_id| handlers::get_category(category_id, state.clone()))
}

/// Returns a random `Category`.
fn random(state: State) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("random" / "categories")
        .and(warp::get())
        .and_then(move || handlers::random(state.clone()))
}

/// Searches for categorys matching the given search term.
fn search(state: State) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("search" / "categories" / String)
        .and(warp::get())
        .and_then(move |search_term| handlers::search(search_term, state.clone()))
}

mod handlers {
    use super::{State, VaultError};
    use serde::Serialize;
    use vault_graph::{Category, Graph, Random, Search};
    use warp::{reject, reply, Rejection, Reply};

    /// Returns a list of all categories.
    pub async fn get_categories(state: State) -> Result<impl Reply, Rejection> {
        let mut categories: Vec<&Category> = state.categories().values().collect();

        categories.sort_unstable_by_key(|category| category.category.as_str());
        Ok(reply::json(&categories))
    }

    /// Returns the `Category` with the given id, if found.
    ///
    /// # Errors
    /// * Returns a `404` error if no `Category` with the given id is found.
    pub async fn get_category(category_id: String, state: State) -> Result<impl Reply, Rejection> {
        match state.categories().get(&category_id) {
            None => Err(reject::custom(VaultError::CategoryNotFound(category_id))),

            Some(category) => Ok(reply::json(&CategoryResponse::new(category, &state))),
        }
    }

    /// Returns a random `Category`.
    pub async fn random(state: State) -> Result<impl Reply, Rejection> {
        Ok(reply::json(&CategoryResponse::new(
            state.categories().random(),
            &state,
        )))
    }

    /// Searches for categorys matching the given search term.
    pub async fn search(search_term: String, state: State) -> Result<impl Reply, Rejection> {
        Ok(reply::json(
            &state
                .category_names()
                .search(&search_term, state.categories()),
        ))
    }

    #[derive(Serialize)]
    /// A struct containing a `Category` as well as any subcategories.
    pub struct CategoryResponse<'a> {
        /// The `Category` in question.
        category: &'a Category,

        /// A list of any subcategories of the given `Category`.
        children: Vec<&'a Category>,
    }

    impl<'a> CategoryResponse<'a> {
        /// Creates a `CategoryResponse` from the given `Category.
        ///
        /// # Arguments
        /// * `category` - the given `Category`.
        /// * `graph` - the `Graph` containing the crates.io data.
        #[must_use]
        pub fn new(category: &'a Category, graph: &'a Graph) -> CategoryResponse<'a> {
            CategoryResponse {
                category,
                children: graph
                    .categories()
                    .values()
                    .filter(|list_category| {
                        list_category.category != category.category
                            && list_category
                                .category
                                .starts_with(category.category.as_str())
                    })
                    .collect(),
            }
        }
    }
}
