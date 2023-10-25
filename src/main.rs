extern crate mongodb;
mod config;
mod errors;
mod database;

use axum::{ 
    routing::get,
    Router, 
    response::{ Html, IntoResponse },
    http::StatusCode,
    extract::State
};
use mongodb::{ Client, bson::doc };
use askama::Template;
use errors::CustomError;
use tower_http::services::ServeDir;
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

    let user_doc = database::get_document(&client, database_name, collection_name, filter).await;
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
    let client = database::connect_database(auth_string).await;

    let shared_state = Arc::new(client);

    let app = Router::new()
                            .route("/", get(index)
                            .with_state(shared_state))
                            .nest_service("/assets", ServeDir::new("assets"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("[+] Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}