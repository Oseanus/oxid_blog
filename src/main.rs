extern crate mongodb;

mod config;

use axum::{ routing::get, Router };
use mongodb::{ Client, options::ClientOptions, bson::{doc, Document} };
use std::net::SocketAddr;

async fn index() -> String {
    String::from("Oxid Blog")
}

#[tokio::main]
async fn main() {
    let config = config::Config::new(String::from("127.0.0.1"), 27017, String::from("oliver"), String::from("Stoneenge"));
    let auth_string = config.get_auth_string();

    let client_options = ClientOptions::parse(auth_string).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let database_name = "blog";
    let collection_name = "users";
    let filter = doc! {"name": "Oliver"};

    let user = get_document(&client, database_name, collection_name, filter).await;

    match user {
        Some(document) => println!("{}", document),
        None => println!("No document found."),
    }

    let app = Router::new().route("/", get(index));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("[+] Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn insert_document(client: &Client, database_name: &str, collection_name: &str, document: Document) {
    let database = client.database(database_name);
    let collection = database.collection(collection_name);
    collection.insert_one(document, None).await.unwrap();
}

async fn get_document(client: &Client, database_name: &str, collection_name: &str, filter: Document) -> Option<Document> {
    let database = client.database(database_name);
    let collection = database.collection::<Document>(collection_name);
    let result = collection.find_one(filter, None).await.unwrap();
    
    result
}