use vault_db::ingest::{fs as vault_fs, load as vault_load};

#[tokio::main]
async fn main() {
    let import_path = vault_fs::fetch_data();

    let mut data_path = vault_fs::get_data_path(&import_path).unwrap();

    dotenv::dotenv().unwrap();

    vault_load::load_database(data_path.split_off(import_path.len() + 1).as_str())
        .await
        .unwrap();

    // vault_fs::clean_tempdir(import_path);
}
