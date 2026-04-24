use axum::response::Html;
use rsx_macros::rsx;
use rsxui::components::{Color, Link, LinkStyle};

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <Link href="https://example.com">"I am a simple link"</Link>
    };

    let hover_example = rsx! {
        <Link href="/" style=LinkStyle::Hover>"I have hover style"</Link>
    };

    let colors_example = rsx! {
        <div class="flex flex-wrap gap-4">
            <Link href="/" color=Color::Primary>"Primary"</Link>
            <Link href="/" color=Color::Secondary>"Secondary"</Link>
            <Link href="/" color=Color::Accent>"Accent"</Link>
            <Link href="/" color=Color::Neutral>"Neutral"</Link>
            <Link href="/" color=Color::Info>"Info"</Link>
            <Link href="/" color=Color::Success>"Success"</Link>
            <Link href="/" color=Color::Warning>"Warning"</Link>
            <Link href="/" color=Color::Error>"Error"</Link>
        </div>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Link"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Link adds the missing underline style to links."
            </p>

            <div class="divider">"Basic Link"</div>
            {basic_example}

            <div class="divider">"Hover Style"</div>
            <p class="text-sm text-gray-600 mb-4">"Only shows underline on hover."</p>
            {hover_example}

            <div class="divider">"Colors"</div>
            <p class="text-sm text-gray-600 mb-4">"Different semantic colors."</p>
            {colors_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Link, LinkStyle, Color};",
                "use rsx_macros::rsx;",
                "",
                "let html = rsx! {",
                "    <Link href=\"https://example.com\" color=Color::Primary>",
                "        \"Click me\"",
                "    </Link>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/link").await)
}
