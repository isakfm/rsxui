use axum::response::Html;
use rsx_macros::rsx;
use rsxui::components::{Color, Divider};

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <Divider />
    };

    let text_example = rsx! {
        <Divider text="OR" />
    };

    let color_example = rsx! {
        <div class="flex flex-col gap-4">
            <Divider text="Primary" color=Color::Primary  />
            <Divider text="Secondary" color=Color::Secondary  />
            <Divider text="Accent" color=Color::Accent  />
            <Divider text="Success" color=Color::Success  />
        </div>
    };

    let vertical_example = rsx! {
        <div class="flex h-32">
            <div class="flex-1">"Content Left"</div>
            <Divider vertical=true />
            <div class="flex-1">"Content Right"</div>
        </div>
    };

    let placement_example = rsx! {
        <div class="flex flex-col gap-4">
            <Divider text="Start" start=true />
            <Divider text="Center" />
            <Divider text="End" end=true />
        </div>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Divider"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Divider is used to separate content vertically or horizontally."
            </p>

            <div class="divider">"Basic Divider"</div>
            {basic_example}

            <div class="divider">"With Text"</div>
            {text_example}

            <div class="divider">"Colors"</div>
            <p class="text-sm text-gray-600 mb-4">"Different semantic colors."</p>
            {color_example}

            <div class="divider">"Vertical"</div>
            <p class="text-sm text-gray-600 mb-4">"Vertical divider between content."</p>
            {vertical_example}

            <div class="divider">"Placement"</div>
            <p class="text-sm text-gray-600 mb-4">"Text alignment: start, center, end."</p>
            {placement_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Divider, Color};",
                "use rsx_macros::rsx;",
                "",
                "let html = rsx! {",
                "    <Divider />",
                "};",
                "",
                "// With text and color",
                "let html = rsx! {",
                "    <Divider text=\"OR\" color={Color::Primary} />",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/divider").await)
}
