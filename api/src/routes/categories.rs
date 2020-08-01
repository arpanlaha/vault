use super::super::utils::State;
use actix_web::{HttpRequest, HttpResponse};
use serde::Serialize;
use vault_graph::{Category, Graph, Random, Search};

#[derive(Serialize)]
pub struct CategoryResponse<'a> {
    category: &'a Category,
    children: Vec<&'a Category>,
}

impl<'a> CategoryResponse<'a> {
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

pub async fn get_categories(data: State) -> HttpResponse {
    let graph = data.read().await;
    let mut categories = graph.categories().values().collect::<Vec<&Category>>();
    categories.sort_unstable_by_key(|category| category.category.as_str());

    HttpResponse::Ok().json(categories)
}

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

pub async fn random(data: State) -> HttpResponse {
    let graph = data.read().await;
    let category = graph.categories().random();

    HttpResponse::Ok().json(CategoryResponse::new(category, &graph))
}

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
