use arangors::{client::reqwest::ReqwestClient, ClientError, Connection, Database};

pub async fn get_connection() -> Result<Connection, ClientError> {
    println!("Establishing driver connection...");
    Connection::establish_jwt(
        dotenv::var("ARANGODB_URI").unwrap().as_str(),
        dotenv::var("ARANGODB_USER").unwrap().as_str(),
        dotenv::var("ARANGODB_PASSWORD").unwrap().as_str(),
    )
    .await
}

pub async fn get_db<'a, 'b>(
    connection: &'a Connection,
    db_name: &'b str,
) -> Result<Database<'a, ReqwestClient>, ClientError> {
    connection.db(db_name).await
}

pub async fn create_collections<'a>(
    db: &mut Database<'a, ReqwestClient>,
    collection_names: Vec<&str>,
) -> Result<(), ClientError> {
    for collection_name in collection_names {
        db.create_collection(collection_name).await?;
    }

    Ok(())
}

pub async fn truncate_collections<'a>(
    _db: &Database<'a, ReqwestClient>,
    collection_names: Vec<&str>,
) -> Result<(), ClientError> {
    for _collection_name in collection_names {
        //* waiting on implementation
        // db.collection(collection_name).await?.truncate().await?;
    }

    Ok(())
}

pub async fn drop_collections<'a>(
    db: &mut Database<'a, ReqwestClient>,
    collection_names: Vec<&str>,
) -> Result<(), ClientError> {
    for collection_name in collection_names {
        db.drop_collection(collection_name).await?;
    }

    Ok(())
}
