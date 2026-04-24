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
        .route(
            "/components/browser_mockup",
            get(pages::browser_mockup::page),
        )
        .route("/components/breadcrumb", get(pages::breadcrumb::page))
        .route("/components/button", get(pages::button::page))
        .route("/components/card", get(pages::card::page))
        .route("/components/chat", get(pages::chat::page))
        .route("/components/checkbox", get(pages::checkbox::page))
        .route("/components/code_mockup", get(pages::code_mockup::page))
        .route("/components/collapse", get(pages::collapse::page))
        .route("/components/countdown", get(pages::countdown::page))
        .route("/components/diff", get(pages::diff::page))
        .route("/components/divider", get(pages::divider::page))
        .route("/components/dock", get(pages::dock::page))
        .route("/components/drawer", get(pages::drawer::page))
        .route("/components/dropdown", get(pages::dropdown::page))
        .route("/components/fieldset", get(pages::fieldset::page))
        .route("/components/file_input", get(pages::file_input::page))
        .route("/components/filter", get(pages::filter::page))
        .route("/components/footer", get(pages::footer::page))
        .route("/components/hero", get(pages::hero::page))
        .route("/components/indicator", get(pages::indicator::page))
        .route("/components/input", get(pages::input::page))
        .route("/components/join", get(pages::join::page))
        .route("/components/kbd", get(pages::kbd::page))
        .route("/components/label", get(pages::label::page))
        .route("/components/list", get(pages::list::page))
        .route("/components/loading", get(pages::loading::page))
        .route("/components/menu", get(pages::menu::page))
        .route("/components/phone_mockup", get(pages::phone_mockup::page))
        .route("/components/progress", get(pages::progress::page))
        .route("/components/radio", get(pages::radio::page))
        .route("/components/range", get(pages::range::page))
        .route("/components/rating", get(pages::rating::page))
        .route("/components/select", get(pages::select::page))
        .route("/components/skeleton", get(pages::skeleton::page))
        .route("/components/stat", get(pages::stat::page))
        .route("/components/status", get(pages::status::page))
        .route("/components/swap", get(pages::swap::page))
        .route("/components/table", get(pages::table::page))
        .route("/components/textarea", get(pages::textarea::page))
        .route("/components/textrotate", get(pages::textrotate::page))
        .route("/components/toggle", get(pages::toggle::page))
        .route("/components/validator", get(pages::validator::page))
        .route("/components/window_mockup", get(pages::window_mockup::page))
        .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
