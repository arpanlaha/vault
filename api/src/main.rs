use actix_cors::Cors;
use actix_web::{
    middleware::{Compress, Logger},
    web::{self, Data},
    App, HttpServer,
};
use env_logger::Env;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::io::Result as IoResult;
use tokio::sync::RwLock;
use vault_api::server::{
    crates, reset,
    state::{AppState, Graph},
};

#[actix_rt::main]
async fn main() -> IoResult<()> {
    dotenv::dotenv().unwrap();

    let app_state = Data::new(AppState {
        graph: RwLock::new(Graph::new().await),
    });

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(
            dotenv::var("SSL_PRIVATE_KEY_PATH").expect("SSL private key path not provided"),
            SslFiletype::PEM,
        )
        .unwrap();
    builder
        .set_certificate_chain_file(
            dotenv::var("SSL_CERT_PATH").expect("SSL certificate chain path not provided"),
        )
        .unwrap();

    env_logger::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
            .wrap(Logger::default())
            .wrap(Cors::default())
            .route(
                "graph/{crate_id}",
                web::get().to(crates::get_dependency_graph),
            )
            .route("crates/{crate_id}", web::get().to(crates::get_crate))
            .route("search/crates/{search_term}", web::get().to(crates::search))
            .route("reset", web::put().to(reset::reset_state))
            .app_data(app_state.clone())
    })
    .bind_openssl("0.0.0.0:443", builder)?
    .run()
    .await
}
