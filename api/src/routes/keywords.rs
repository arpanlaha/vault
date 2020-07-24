use super::super::utils::{
    common::{Random, Search},
    state::AppState,
};
use actix_web::{web::Data, HttpRequest, HttpResponse, Responder};

pub async fn get_keyword(req: HttpRequest, data: Data<AppState>) -> impl Responder {
    match req.match_info().get("keyword_id") {
        None => HttpResponse::BadRequest().json("Keyword id must be provided."),

        Some(keyword_id) => match data.graph.read().await.keywords().get(keyword_id) {
            None => HttpResponse::NotFound()
                .json(format!("Keyword with id {} does not exist.", keyword_id)),

            Some(keyword) => HttpResponse::Ok().json(keyword),
        },
    }
}

pub async fn random(data: Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(data.graph.read().await.keywords().random())
}

pub async fn search(req: HttpRequest, data: Data<AppState>) -> impl Responder {
    match req.match_info().get("search_term") {
        None => HttpResponse::BadRequest().json("Search term must be provided."),

        Some(search_term) => {
            HttpResponse::Ok().json(data.graph.read().await.keywords().search(search_term))
        }
    }
}
