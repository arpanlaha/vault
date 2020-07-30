use super::super::utils::state::AppState;
use actix_web::{web::Data, HttpRequest, HttpResponse};
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

pub async fn get_categories(data: Data<AppState>) -> HttpResponse {
    let graph = data.graph.read().await;
    let mut categories = graph.categories().values().collect::<Vec<&Category>>();
    categories.sort_unstable_by_key(|category| category.category.as_str());

    HttpResponse::Ok().json(categories)
}

pub async fn get_category(req: HttpRequest, data: Data<AppState>) -> HttpResponse {
    match req.match_info().get("category_id") {
        None => HttpResponse::BadRequest().json("Category id must be provided."),

        Some(category_id) => {
            let graph = data.graph.read().await;
            match graph.categories().get(category_id) {
                None => HttpResponse::NotFound()
                    .json(format!("Category with id {} does not exist.", category_id)),

                Some(category) => HttpResponse::Ok().json(CategoryResponse::new(category, &graph)),
            }
        }
    }
}

pub async fn random(data: Data<AppState>) -> HttpResponse {
    let graph = data.graph.read().await;
    let category = graph.categories().random();

    HttpResponse::Ok().json(CategoryResponse::new(category, &graph))
}

pub async fn search(req: HttpRequest, data: Data<AppState>) -> HttpResponse {
    match req.match_info().get("search_term") {
        None => HttpResponse::BadRequest().json("Search term must be provided."),

        Some(search_term) => {
            HttpResponse::Ok().json(data.graph.read().await.categories().search(search_term))
        }
    }
}
