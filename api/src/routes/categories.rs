use super::super::{
    ingest::schema::Category,
    utils::{
        common::{Random, Search},
        state::AppState,
    },
};
use actix_web::{web::Data, HttpRequest, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct CategoryResponse<'a> {
    category: &'a Category,
    children: Vec<&'a Category>,
}

pub async fn get_categories(data: Data<AppState>) -> impl Responder {
    let graph = data.graph.read().await;
    let mut categories = graph.categories().values().collect::<Vec<&Category>>();
    categories.sort_by(|a, b| a.category.partial_cmp(&b.category).unwrap());

    HttpResponse::Ok().json(categories)
}

pub async fn get_category(req: HttpRequest, data: Data<AppState>) -> impl Responder {
    match req.match_info().get("category_id") {
        None => HttpResponse::BadRequest().json("Category id must be provided."),

        Some(category_id) => {
            let graph = data.graph.read().await;
            match graph.categories().get(category_id) {
                None => HttpResponse::NotFound()
                    .json(format!("Category with id {} does not exist.", category_id)),

                Some(category) => HttpResponse::Ok().json(CategoryResponse {
                    category,
                    children: graph
                        .categories()
                        .values()
                        .filter(|Category { category, .. }| {
                            category != category_id && category.starts_with(category_id)
                        })
                        .collect(),
                }),
            }
        }
    }
}

pub async fn random(data: Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(data.graph.read().await.categories().random())
}

pub async fn search(req: HttpRequest, data: Data<AppState>) -> impl Responder {
    match req.match_info().get("search_term") {
        None => HttpResponse::BadRequest().json("Search term must be provided."),

        Some(search_term) => {
            HttpResponse::Ok().json(data.graph.read().await.categories().search(search_term))
        }
    }
}
