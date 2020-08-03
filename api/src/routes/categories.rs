use super::super::utils::State;
use actix_web::{HttpRequest, HttpResponse};
use serde::Serialize;
use vault_graph::{Category, Graph, Random, Search};

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

/// Returns a list of all categories.
pub async fn get_categories(data: State) -> HttpResponse {
    let graph = data.read().await;
    let mut categories = graph.categories().values().collect::<Vec<&Category>>();
    categories.sort_unstable_by_key(|category| category.category.as_str());

    HttpResponse::Ok().json(categories)
}

/// Returns the `Category` with the given id, if found.
///
/// # Errors
/// * Returns a `400` error if `category_id` is not present.
/// * Returns a `404` error if no `Category` with the given id is found.
pub async fn get_category(req: HttpRequest, data: State) -> HttpResponse {
    match req.match_info().get("category_id") {
        None => HttpResponse::BadRequest().json("Category id must be provided."),

        Some(category_id) => {
            let graph = data.read().await;
            match graph.categories().get(category_id) {
                None => HttpResponse::NotFound()
                    .json(format!("Category with id {} does not exist.", category_id)),

                Some(category) => HttpResponse::Ok().json(CategoryResponse::new(category, &graph)),
            }
        }
    }
}

/// Returns a random `Category`.
pub async fn random(data: State) -> HttpResponse {
    let graph = data.read().await;
    let category = graph.categories().random();

    HttpResponse::Ok().json(CategoryResponse::new(category, &graph))
}

/// Searches for categories matching the given search term.
///
/// # Errors
/// * Returns a `400` error if `search_term` is not present.
pub async fn search(req: HttpRequest, data: State) -> HttpResponse {
    match req.match_info().get("search_term") {
        None => HttpResponse::BadRequest().json("Search term must be provided."),

        Some(search_term) => {
            let graph = data.read().await;
            HttpResponse::Ok().json(
                graph
                    .category_names()
                    .search(search_term, graph.categories()),
            )
        }
    }
}
