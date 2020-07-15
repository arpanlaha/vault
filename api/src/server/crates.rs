use super::graph::AppState;
use actix_web::{web::Data, HttpRequest, HttpResponse, Responder};

pub async fn get_transitive_dependencies_by_crate_id(
    req: HttpRequest,
    data: Data<AppState>,
) -> impl Responder {
    match req.match_info().get("crate_id") {
        None => HttpResponse::BadRequest().json("Crate id must be provided."),

        Some(crate_id) => match crate_id.parse::<usize>() {
            Err(_) => HttpResponse::BadRequest()
                .json(format!("Could not parse {} as an integer.", crate_id)),

            Ok(crate_id) => match &data.graph.read().await.transitive_dependencies(crate_id) {
                None => HttpResponse::NotFound()
                    .json(format!("Crate with id {} does not exist.", crate_id)),

                Some(dependencies) => HttpResponse::Ok().json(dependencies),
            },
        },
    }
}
