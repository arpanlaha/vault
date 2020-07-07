use redis::Client;
use redisgraph::{Graph, RedisGraphResult};

use vault_db::ingest::{fs as vault_fs, load as vault_load};

#[tokio::main]
async fn main() {
    let client = Client::open("redis://127.0.0.1").unwrap();
    let mut connection = client.get_connection().unwrap();
    let graph = Graph::open(connection, String::from("vault"));
    // let temp_dir = vault_fs::fetch_data();

    // let data_path = vault_fs::get_data_path(&temp_dir).expect("Unable to locate data path");

    // dotenv::dotenv().unwrap();

    // vault_load::load_database(data_path.as_str()).await.unwrap();

    // vault_fs::clean_tempdir(temp_dir);
}
