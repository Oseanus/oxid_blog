extern crate mongodb;

use axum::{ routing::get, Router , http::StatusCode};
use mongodb::{ Client, options::ClientOptions, bson::{doc, Document}, Database };
use std::net::SocketAddr;

async fn index() -> String {
    String::from("Oxid Blog")
}

#[tokio::main]
async fn main() {
    let client_options = ClientOptions::parse("mongodb://oliver:Stoneenge@127.0.0.1:27017").await.unwrap();
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

    // match result {
    //     Some(document) => println!("{}", document),
    //     None => println!("No document found."),
    // }

    result
}