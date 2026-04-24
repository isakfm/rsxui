use axum::response::Html;
use rsx_macros::rsx;
use rsxui::components::{Alert, Color};

use crate::layout;

pub async fn page() -> Html<String> {
    let color_examples = rsx! {
        <div class="flex flex-col gap-4">
            <Alert>"Default alert"</Alert>
            <Alert color=Color::Info>"Info alert"</Alert>
            <Alert color=Color::Success>"Success alert"</Alert>
            <Alert color=Color::Warning>"Warning alert"</Alert>
            <Alert color=Color::Error>"Error alert"</Alert>
        </div>
    };

    let soft_examples = rsx! {
        <div class="flex flex-col gap-4">
            <Alert color=Color::Info soft=true>"Soft info alert"</Alert>
            <Alert color=Color::Success soft=true>"Soft success alert"</Alert>
            <Alert color=Color::Warning soft=true>"Soft warning alert"</Alert>
            <Alert color=Color::Error soft=true>"Soft error alert"</Alert>
        </div>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Alert"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Alerts display important messages to users."
            </p>

            <div class="divider">"Colors"</div>
            {color_examples}

            <div class="divider">"Soft Style"</div>
            {soft_examples}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Alert, Color};",
                "use rsx_macros::rsx;",
                "",
                "let html = rsx! {",
                "    <Alert color=Color::Success>\"Operation completed!\"</Alert>",
                "    <Alert color=Color::Error soft=true>\"Soft error\"</Alert>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/alert").await)
}
