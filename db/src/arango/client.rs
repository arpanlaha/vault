// use redis::{Client, Connection, RedisResult};
// use redisgraph::{Graph, RedisGraphResult};
use bolt_client::{error::Result as BoltResult, Client, Metadata};
use bolt_proto::message::{Record, Success};
use std::convert::TryFrom;
use std::iter::FromIterator;
// use bolt_proto::{message::*, value::*, Message, Value};

//BoltResult<(Message, Vec<Record>)>
pub async fn run_query(client: &mut Client, query: &str) -> BoltResult<Vec<Record>> {
    Success::try_from(client.run_with_metadata(query, None, None).await?)?;
    let (pull_response, records) = client.pull(None).await?;
    Success::try_from(pull_response)?;

    Ok(records)
}

pub async fn get_connection() -> BoltResult<Client> {
    println!("Establishing driver connection...");
    let mut client = Client::new(
        dotenv::var("NEO4J_ADDRESS").unwrap(),
        None::<String>, // dotenv::var("NEO4J_DOMAIN").ok(),
    )
    .await?;

    client.handshake(&[4, 0, 0, 0]).await?;

    client
        .hello(Some(Metadata::from_iter(vec![
            ("user_agent", "my-client-name/1.0"),
            ("scheme", "basic"),
            ("principal", &dotenv::var("NEO4J_USER").unwrap()),
            ("credentials", &dotenv::var("NEO4J_PASSWORD").unwrap()),
        ])))
        .await?;
    // let connection = Client::open("redis://127.0.0.1")?.get_connection();
    println!("Established driver connection.");
    Ok(client)
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
