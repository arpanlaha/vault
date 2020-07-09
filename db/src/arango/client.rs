use redis::{Client, Connection, RedisResult};
use redisgraph::{Graph, RedisGraphResult};

pub fn get_connection() -> RedisResult<Connection> {
    println!("Establishing driver connection...");
    let connection = Client::open("redis://127.0.0.1")?.get_connection();
    println!("Established driver connection.");
    connection
}

pub fn get_db(connection: Connection, db_name: &str) -> RedisGraphResult<Graph> {
    Graph::open(connection, String::from(db_name))
}

// pub async fn create_collections<'a>(
//     db: &mut Database<'a, ReqwestClient>,
//     collection_names: Vec<&str>,
// ) -> Result<(), ClientError> {
//     for collection_name in collection_names {
//         db.create_collection(collection_name).await?;
//     }

//     Ok(())
// }

// pub async fn truncate_collections<'a>(
//     _db: &Database<'a, ReqwestClient>,
//     collection_names: Vec<&str>,
// ) -> Result<(), ClientError> {
//     for _collection_name in collection_names {
//         //* waiting on implementation
//         // db.collection(collection_name).await?.truncate().await?;
//     }

//     Ok(())
// }

pub fn drop_collections<'a>(db: &mut Graph, collection_names: Vec<&str>) -> RedisGraphResult<()> {
    for collection_name in collection_names {
        db.mutate(format!(r#"MATCH (n:{}) DETACH DELETE n"#, collection_name).as_str())?;
    }

    Ok(())
}
