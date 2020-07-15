use super::graph::AppState;
use actix_web::{web::Data, HttpRequest, HttpResponse, Responder};

pub async fn get_transitive_dependencies_by_crate_id(
    req: HttpRequest,
    data: Data<AppState>,
) -> impl Responder {
    HttpResponse::Ok().content_type("application/json").body(
        serde_json::to_string(
            &data.graph.read().await.transitive_dependencies(
                req.match_info()
                    .get("crate_id")
                    .expect("crate_id not provided")
                    .parse()
                    .expect("Unable to parse crate_id as integer"),
            ),
        )
        .expect("Unable to serialize crates"),
    )
}
