use actix_cors::Cors;
use actix_web::{
    middleware::{Compress, Logger},
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use dotenv_codegen::dotenv;
use env_logger::Env;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::io::Result as IoResult;
use std::time::Instant;
use tokio::sync::{Mutex, RwLock};
use vault_api::server::{
    categories, crates, keywords, reset,
    state::{AppState, Graph},
};

#[actix_rt::main]
async fn main() -> IoResult<()> {
    let app_state = Data::new(AppState {
        graph: RwLock::new(Graph::new().await),
        last_updated: Mutex::new(Instant::now()),
    });

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(dotenv!("SSL_PRIVATE_KEY_PATH"), SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_chain_file(dotenv!("SSL_CERT_PATH"))
        .unwrap();

    env_logger::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
            .wrap(Logger::default())
            .wrap(Cors::default())
            .app_data(app_state.clone())
            .route(
                "graph/{crate_id}",
                web::get().to(crates::get_dependency_graph),
            )
            .route("crates/{crate_id}", web::get().to(crates::get_crate))
            .route("categories", web::get().to(categories::get_categories))
            .route(
                "categories/{category_id}",
                web::get().to(categories::get_category),
            )
            .route(
                "keywords/{keyword_id}",
                web::get().to(keywords::get_keyword),
            )
            .service(
                web::scope("random")
                    .route("crates", web::get().to(crates::random))
                    .route("categories", web::get().to(categories::random))
                    .route("keywords", web::get().to(keywords::random)),
            )
            .service(
                web::scope("search")
                    .route("crates/{search_term}", web::get().to(crates::search))
                    .route(
                        "categories/{search_term}",
                        web::get().to(categories::search),
                    )
                    .route("keywords/{search_term}", web::get().to(keywords::search)),
            )
            .route("reset", web::put().to(reset::reset_state))
            .default_service(web::route().to(|| HttpResponse::NotFound().json("Route not found")))
    })
    .bind_openssl("0.0.0.0:443", builder)?
    .run()
    .await
}
