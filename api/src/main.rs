use actix_web::{
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use std::io::Result as IoResult;
use tokio::sync::RwLock;
use vault_api::ingest::{fs as vault_fs, load as vault_load, traits::Graph};

struct AppState {
    graph: RwLock<Graph>,
}

async fn index(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
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
            .route("/{crate_id}", web::get().to(index))
            .app_data(app_state.clone())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await

    // vault_fs::clean_tempdir(temp_dir);
}
