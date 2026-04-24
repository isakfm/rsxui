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
        .route("/components/checkbox", get(pages::checkbox::page))
        .route("/components/collapse", get(pages::collapse::page))
        .route("/components/countdown", get(pages::countdown::page))
        .route("/components/diff", get(pages::diff::page))
        .route("/components/divider", get(pages::divider::page))
        .route("/components/drawer", get(pages::drawer::page))
        .route("/components/fieldset", get(pages::fieldset::page))
        .route("/components/file_input", get(pages::file_input::page))
        .route("/components/filter", get(pages::filter::page))
        .route("/components/input", get(pages::input::page))
        .route("/components/label", get(pages::label::page))
        .route("/components/menu", get(pages::menu::page))
        .route("/components/radio", get(pages::radio::page))
        .route("/components/range", get(pages::range::page))
        .route("/components/rating", get(pages::rating::page))
        .route("/components/select", get(pages::select::page))
        .route("/components/textarea", get(pages::textarea::page))
        .route("/components/toggle", get(pages::toggle::page))
        .route("/components/validator", get(pages::validator::page))
        .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
