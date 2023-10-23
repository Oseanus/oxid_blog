extern crate mongodb;
mod config;
mod errors;

use axum::{ routing::get, Router, response::{Html, IntoResponse}, http::StatusCode, extract::{ State } };
use mongodb::{ Client, options::ClientOptions, bson::{doc, Document} };
use askama::Template;
use errors::CustomError;
use std::net::SocketAddr;
use std::sync::Arc;

#[derive(Template)]
#[template(path = "users.html")]
struct User<'a> {
    title: String,
    name: &'a str,
    email: &'a str
}

async fn index(State(state): State<Arc<Client>>) -> impl IntoResponse {
    let client = state.clone();

    let database_name = "blog";
    let collection_name = "users";
    let filter = doc! {"name": "Oliver"};

    let user_doc = get_document(&client, database_name, collection_name, filter).await;
    let user = user_doc.unwrap();

    let template = User {
        title: String::from("Users"),
        name : user.get("name").unwrap().as_str().unwrap(),
        email: user.get("email").unwrap().as_str().unwrap()
    };

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render template. Error {}", err),
        ).into_response(),
    }
}

#[tokio::main]
async fn main() {
    let config = config::Config::new(String::from("127.0.0.1"), 27017, String::from("oliver"), String::from("Stoneenge"));
    let auth_string = config.get_auth_string();
    let client = connect_database(auth_string).await;

    let shared_state = Arc::new(client);

    let app = Router::new()
                            .route("/", get(index)
                            .with_state(shared_state));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("[+] Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn connect_database(auth_string: String) -> Client {
    let client_options = ClientOptions::parse(auth_string).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    client
}

async fn insert_document(client: &Client, database_name: &str, collection_name: &str, document: Document) {
    let database = client.database(database_name);
    let collection = database.collection(collection_name);
    collection.insert_one(document, None).await.unwrap();
}

async fn get_document(client: &Client, database_name: &str, collection_name: &str, filter: Document) -> Result<Document, CustomError> {
    let database = client.database(database_name);
    let collection = database.collection::<Document>(collection_name);
    let result = collection.find_one(filter, None).await.unwrap();

    result.ok_or_else(|| CustomError::Database("Error: Failed to get document from database.".to_string()))
}