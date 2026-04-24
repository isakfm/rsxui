use axum::response::Html;
use rsx_macros::rsx;
use rsxui::components::{Color, Progress};

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <div class="flex flex-col gap-2">
            <Progress value="0" max="100" class="w-56" />
            <Progress value="10" max="100" class="w-56" />
            <Progress value="40" max="100" class="w-56" />
            <Progress value="70" max="100" class="w-56" />
            <Progress value="100" max="100" class="w-56" />
        </div>
    };

    let colors_example = rsx! {
        <div class="flex flex-col gap-2">
            <Progress value="40" max="100" color=Color::Primary class="w-56" />
            <Progress value="50" max="100" color=Color::Secondary class="w-56" />
            <Progress value="60" max="100" color=Color::Accent class="w-56" />
            <Progress value="70" max="100" color=Color::Success class="w-56" />
            <Progress value="80" max="100" color=Color::Warning class="w-56" />
            <Progress value="90" max="100" color=Color::Error class="w-56" />
        </div>
    };

    let indeterminate_example = rsx! {
        <Progress class="w-56" />
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Progress"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Progress bar can be used to show the progress of a task or to show the passing of time."
            </p>

            <div class="divider">"Basic Progress"</div>
            {basic_example}

            <div class="divider">"Colors"</div>
            <p class="text-sm text-gray-600 mb-4">"Different semantic colors."</p>
            {colors_example}

            <div class="divider">"Indeterminate"</div>
            <p class="text-sm text-gray-600 mb-4">"Without a value, it shows an indeterminate animation."</p>
            {indeterminate_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Progress, Color};",
                "use rsx_macros::rsx;",
                "",
                "let html = rsx! {",
                "    <Progress value=\"70\" max=\"100\" color={Color::Primary} />",
                "};",
                "",
                "// Indeterminate",
                "let html = rsx! {",
                "    <Progress />",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/progress").await)
}
