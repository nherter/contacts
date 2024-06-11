use std::fs;

use askama::Template;
use axum::{
    extract::Path,
    http::{response, StatusCode},
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};

use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(get_contacts).post(create_contact))
        .route("/:index", delete(delete_contact));

    let listener = tokio::net::TcpListener::bind("server:3001").await.unwrap();
    println!("Listening on http:/localhost:3001");
    axum::serve(listener, app).await.unwrap();
}

async fn get_contacts() -> impl IntoResponse {
    let content = fs::read_to_string("./contacts.json").unwrap();

    let response: Vec<Contact> = serde_json::from_str(&content).unwrap();

    Json(response)
}

async fn create_contact(Json(contact): Json<Contact>) -> impl IntoResponse {
    let read_file = fs::read_to_string("./contacts.json").unwrap();

    let mut contacts: Vec<Contact> = serde_json::from_str(&read_file).unwrap();
    contacts.push(contact);

    let into_json = serde_json::to_string(&contacts).unwrap();

    fs::write("./contacts.json", into_json).unwrap();

    StatusCode::CREATED
}

async fn delete_contact(Path((index)): Path<usize>) -> impl IntoResponse {
    let read_file = fs::read_to_string("./contacts.json").unwrap();

    let mut contacts: Vec<Contact> = serde_json::from_str(&read_file).unwrap();
    contacts.remove(index);

    let into_json = serde_json::to_string(&contacts).unwrap();

    fs::write("./contacts.json", into_json).unwrap();

    StatusCode::OK
}

#[derive(Serialize, Deserialize, Debug)]
struct Contact {
    name: String,
    email: String,
}
