use actix_web::{
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use std::io::Result as IoResult;
use tokio::sync::RwLock;
use vault_api::ingest::{fs as vault_fs, load as vault_load, traits::Graph};

struct AppState {
    graph: RwLock<Graph>,
}

async fn index(data: web::Data<AppState>) -> impl Responder {
    let graph = data.graph.read().await;
    let dependencies = graph.transitive_dependencies(463);
    // .unwrap()
    // .iter()
    // .map(|dependency| &dependency.name)
    // .collect::<Vec<&String>>();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&dependencies).unwrap())
    // "Hello world!"
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
            .route("/", web::get().to(index))
            .app_data(app_state.clone())
        // .service(web::scope("/").route("/", web::get().to(index)))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await

    // let start = Instant::now();

    // let dependencies = graph.transitive_dependencies(36736).unwrap();

    // println!(
    //     "Found {} transitive dependencies in {} seconds.",
    //     dependencies.len(),
    //     start.elapsed().as_secs_f64()
    // );

    // vault_fs::clean_tempdir(temp_dir);
}
