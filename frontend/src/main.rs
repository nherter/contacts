use std::fs::ReadDir;

use askama::Template;
use axum::{
    extract::{path, Path},
    http::response,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Form, Router,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(get_page).post(post_contact))
        .route("/:index", get(remove_contacts));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn get_page() -> impl IntoResponse {
    let response = reqwest::get("http://127.0.0.1:3001/")
        .await
        .unwrap()
        .json::<Vec<Contact>>()
        .await
        .unwrap();

    let template = IndexTemplate { contact: response };

    template
}

async fn post_contact(Form(contact): Form<Contact>) -> impl IntoResponse {
    let client = reqwest::Client::new();
    let response = client
        .post("http://127.0.0.1:3001/")
        .json(&Contact {
            name: contact.name,
            email: contact.email,
        })
        .send()
        .await
        .unwrap();

    Redirect::to("/")
}

async fn remove_contacts(Path((index)): Path<usize>) -> impl IntoResponse {
    let client = reqwest::Client::new();
    let response = client
        .delete(format!("http://127.0.0.1:3001/{index}"))
        .send()
        .await
        .unwrap();

    Redirect::to("/")
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    contact: Vec<Contact>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Contact {
    name: String,
    email: String,
}
