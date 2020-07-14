use std::time::Instant;
use vault_api::ingest::{fs as vault_fs, load as vault_load};

#[tokio::main]
async fn main() {
    // let temp_dir = vault_fs::fetch_data();

    // let data_path = vault_fs::get_data_path(&temp_dir).unwrap();

    let data_path = String::from("/datadrive/vault/dump/data");

    let graph = vault_load::load_database(data_path.as_str()).await;

    let start = Instant::now();

    let dependencies = graph.transitive_dependencies(36736).unwrap();

    println!(
        "Found {} transitive dependencies in {} seconds.",
        dependencies.len(),
        start.elapsed().as_secs_f64()
    );

    // vault_fs::clean_tempdir(temp_dir);
}
