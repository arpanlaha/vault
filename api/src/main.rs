use actix_web::{
    middleware::{Compress, Logger},
    web::{self, Data},
    App, HttpServer,
};
use env_logger::Env;
use std::io::Result as IoResult;
use tokio::sync::RwLock;
use vault_api::server::{
    crates, reset,
    state::{AppState, Graph},
};

#[actix_rt::main]
async fn main() -> IoResult<()> {
    let app_state = Data::new(AppState {
        graph: RwLock::new(Graph::new().await),
    });

    env_logger::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
            .wrap(Logger::default())
            .route(
                "dependencies/{crate_id}",
                web::get().to(crates::get_transitive_dependencies_by_crate_id),
            )
            .route("crates/{crate_id}", web::get().to(crates::get_crate))
            .route("reset", web::put().to(reset::reset_state))
            .app_data(app_state.clone())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
