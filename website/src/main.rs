use axum::{routing::get, Router};
use tower_http::services::ServeDir;

mod html_utils;
mod layout;
mod pages;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(pages::home::page))
        .route("/components/accordion", get(pages::accordion::page))
        .route("/components/alert", get(pages::alert::page))
        .route("/components/avatar", get(pages::avatar::page))
        .route("/components/badge", get(pages::badge::page))
        .route("/components/button", get(pages::button::page))
        .route("/components/card", get(pages::card::page))
        .route("/components/chat", get(pages::chat::page))
        .route("/components/collapse", get(pages::collapse::page))
        .route("/components/countdown", get(pages::countdown::page))
        .route("/components/drawer", get(pages::drawer::page))
        .route("/components/input", get(pages::input::page))
        .route("/components/menu", get(pages::menu::page))
        .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}