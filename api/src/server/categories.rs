use super::state::AppState;
use actix_web::{web::Data, HttpRequest, HttpResponse, Responder};

pub async fn get_category(req: HttpRequest, data: Data<AppState>) -> impl Responder {
    match req.match_info().get("category_id") {
        None => HttpResponse::BadRequest().json("Category id must be provided."),

        Some(category_id) => match &data.graph.read().await.categories().get(category_id) {
            None => HttpResponse::NotFound()
                .json(format!("Category with id {} does not exist.", category_id)),

            Some(category) => HttpResponse::Ok().json(category),
        },
    }
}

pub async fn search(req: HttpRequest, data: Data<AppState>) -> impl Responder {
    match req.match_info().get("search_term") {
        None => HttpResponse::BadRequest().json("Search term must be provided."),

        Some(search_term) => {
            HttpResponse::Ok().json(data.graph.read().await.category_search(search_term))
        }
    }
}
