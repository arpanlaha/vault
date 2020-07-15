use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use std::io::Result as IoResult;
use tokio::sync::RwLock;
use vault_api::{
    ingest::{fs as vault_fs, load as vault_load},
    server::{crates::get_transitive_dependencies_by_crate_id, graph::AppState},
};

#[actix_rt::main]
async fn main() -> IoResult<()> {
    // let temp_dir = vault_fs::fetch_data();

    // let data_path = vault_fs::get_data_path(&temp_dir).unwrap();

    let data_path = String::from("/datadrive/vault/dump/data");

    let graph = vault_load::load_database(data_path.as_str()).await;

    let app_state = Data::new(AppState {
        graph: RwLock::new(graph),
    });

    HttpServer::new(move || {
        App::new()
            .route(
                "/{crate_id}",
                web::get().to(get_transitive_dependencies_by_crate_id),
            )
            .app_data(app_state.clone())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await

    // vault_fs::clean_tempdir(temp_dir);
}
