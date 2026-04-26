use axum::response::Html;
use rsx::rsx;
use rsxui::components::{Color, Size, Status};

use crate::layout;

pub async fn page() -> Html<String> {
    let sizes_example = rsx! {
        <div class="flex flex-wrap gap-4 items-center">
            <Status size=Size::Xs aria_label="Extra small" />
            <Status size=Size::Sm aria_label="Small" />
            <Status size=Size::Md aria_label="Medium" />
            <Status size=Size::Lg aria_label="Large" />
            <Status size=Size::Xl aria_label="Extra large" />
        </div>
    };

    let colors_example = rsx! {
        <div class="flex flex-wrap gap-4">
            <Status color=Color::Primary aria_label="Primary" />
            <Status color=Color::Secondary aria_label="Secondary" />
            <Status color=Color::Accent aria_label="Accent" />
            <Status color=Color::Neutral aria_label="Neutral" />
            <Status color=Color::Info aria_label="Info" />
            <Status color=Color::Success aria_label="Success" />
            <Status color=Color::Warning aria_label="Warning" />
            <Status color=Color::Error aria_label="Error" />
        </div>
    };

    let ping_example = rsx! {
        <div class="inline-grid *:[grid-area:1/1]">
            <Status color=Color::Error animate="ping" aria_label="Server down" />
            <Status color=Color::Error aria_label="Server down" />
        </div>
        <span class="ml-2">"Server is down"</span>
    };

    let bounce_example = rsx! {
        <div class="flex items-center gap-2">
            <Status color=Color::Info animate="bounce" aria_label="Unread messages" />
            <span>"Unread messages"</span>
        </div>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Status"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Status is a small icon to visually show the current status of an element."
            </p>

            <div class="divider">"Sizes"</div>
            <p class="text-sm text-gray-600 mb-4">"From extra small to extra large."</p>
            {sizes_example}

            <div class="divider">"Colors"</div>
            <p class="text-sm text-gray-600 mb-4">"Different semantic colors."</p>
            {colors_example}

            <div class="divider">"Ping Animation"</div>
            <p class="text-sm text-gray-600 mb-4">"Use animate-ping for attention."</p>
            {ping_example}

            <div class="divider">"Bounce Animation"</div>
            <p class="text-sm text-gray-600 mb-4">"Use animate-bounce for notifications."</p>
            {bounce_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Status, Color, Size};",
                "use rsx::rsx;",
                "",
                "let html = rsx! {",
                "    <Status color={Color::Success} size={Size::Md} aria_label=\"Online\" />",
                "};",
                "",
                "// With animation",
                "let html = rsx! {",
                "    <Status color={Color::Error} animate=\"ping\" />",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/status").await)
}
