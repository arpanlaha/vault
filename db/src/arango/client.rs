// use redis::{Client, Connection, RedisResult};
// use redisgraph::{Graph, RedisGraphResult};

use bolt_client::{Client, Metadata};
use std::iter::FromIterator;
// use bolt_proto::{message::*, value::*, Message, Value};

pub async fn get_connection() -> Client {
    println!("Establishing driver connection...");
    let mut client = Client::new(
        dotenv::var("NEO4J_ADDRESS").unwrap(),
        None::<String>, // dotenv::var("NEO4J_DOMAIN").ok(),
    )
    .await
    .unwrap();
    client.handshake(&[4, 0, 0, 0]).await.unwrap();

    client
        .hello(Some(Metadata::from_iter(vec![
            ("user_agent", "my-client-name/1.0"),
            ("scheme", "basic"),
            ("principal", &dotenv::var("NEO4J_USER").unwrap()),
            ("credentials", &dotenv::var("NEO4J_PASSWORD").unwrap()),
        ])))
        .await
        .unwrap();
    // let connection = Client::open("redis://127.0.0.1")?.get_connection();
    println!("Established driver connection.");
    client
}

// pub fn get_db(connection: Connection, db_name: &str) -> RedisGraphResult<Graph> {
//     Graph::open(connection, String::from(db_name))
// }

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

// pub fn drop_collections<'a>(db: &mut Graph, collection_names: Vec<&str>) -> RedisGraphResult<()> {
//     for collection_name in collection_names {
//         db.mutate(format!(r#"MATCH (n:{}) DETACH DELETE n"#, collection_name).as_str())?;
//     }

//     Ok(())
// }
