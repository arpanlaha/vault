use super::super::utils::State;
use actix_web::{HttpRequest, HttpResponse};
use vault_graph::{Random, Search};

pub async fn get_keyword(req: HttpRequest, data: State) -> HttpResponse {
    match req.match_info().get("keyword_id") {
        None => HttpResponse::BadRequest().json("Keyword id must be provided."),

        Some(keyword_id) => match data.read().await.keywords().get(keyword_id) {
            None => HttpResponse::NotFound()
                .json(format!("Keyword with id {} does not exist.", keyword_id)),

            Some(keyword) => HttpResponse::Ok().json(keyword),
        },
    }
}

pub async fn random(data: State) -> HttpResponse {
    HttpResponse::Ok().json(data.read().await.keywords().random())
}

pub async fn search(req: HttpRequest, data: State) -> HttpResponse {
    match req.match_info().get("search_term") {
        None => HttpResponse::BadRequest().json("Search term must be provided."),

        Some(search_term) => {
            let graph = data.read().await;
            HttpResponse::Ok().json(graph.keyword_names().search(search_term, graph.keywords()))
        }
    }
}
