use actix_web::{web, App, HttpServer};
use std::io::Result as IoResult;
use vault_api::server::{crates, graph};

#[actix_rt::main]
async fn main() -> IoResult<()> {
    let app_state = graph::create_app_state().await;

    HttpServer::new(move || {
        App::new()
            .route(
                "/{crate_id}",
                web::get().to(crates::get_transitive_dependencies_by_crate_id),
            )
            .app_data(app_state.clone())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
