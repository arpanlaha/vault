use vault_api::ingest::{fs as vault_fs, load as vault_load};

#[tokio::main]
async fn main() {
    // let temp_dir = vault_fs::fetch_data();

    // let data_path = vault_fs::get_data_path(&temp_dir).unwrap();

    let data_path = String::from("/datadrive/vault/dump/data");

    let graph = vault_load::load_database(data_path.as_str()).await;

    for dependency in graph.transitive_dependencies(463).unwrap() {
        println!("Dependency: {}", dependency.name);
    }

    // vault_fs::clean_tempdir(temp_dir);
}
