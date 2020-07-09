use vault_db::ingest::{fs as vault_fs, load as vault_load};

#[tokio::main]
async fn main() {
    let temp_dir = vault_fs::fetch_data();

    let data_path = vault_fs::get_data_path(&temp_dir).expect("Unable to locate data path");

    dotenv::dotenv().unwrap();

    vault_load::load_database(data_path.as_str()).await;

    vault_fs::clean_tempdir(temp_dir);
}
