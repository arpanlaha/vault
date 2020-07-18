use actix_web::{
    middleware::Compress,
    web::{self, Data},
    App, HttpServer,
};
use std::io::Result as IoResult;
use tokio::sync::RwLock;
use vault_api::server::{
    crates,
    state::{AppState, Graph},
};

#[actix_rt::main]
async fn main() -> IoResult<()> {
    // let app_state = graph::create_app_state().await;

    let app_state = Data::new(AppState {
        graph: RwLock::new(Graph::new().await),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
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
