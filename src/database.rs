extern crate mongodb;

use mongodb::{ Client, options::ClientOptions, bson::Document };
use crate::CustomError;

pub async fn connect_database(auth_string: String) -> Client {
    let client_options = ClientOptions::parse(auth_string).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    client
}

pub async fn insert_document(client: &Client, database_name: &str, collection_name: &str, document: Document) {
    let database = client.database(database_name);
    let collection = database.collection(collection_name);
    collection.insert_one(document, None).await.unwrap();
}

pub async fn get_document(client: &Client, database_name: &str, collection_name: &str, filter: Document) -> Result<Document, CustomError> {
    let database = client.database(database_name);
    let collection = database.collection::<Document>(collection_name);
    let result = collection.find_one(filter, None).await.unwrap();

    result.ok_or_else(|| CustomError::Database("Error: Failed to get document from database.".to_string()))
}