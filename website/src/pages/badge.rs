use axum::response::Html;
use rsx_macros::rsx;
use rsxui::components::{Badge, BadgeStyle, Color, Size};

use crate::layout;

pub async fn page() -> Html<String> {
    let color_examples = rsx! {
        <div class="flex flex-wrap gap-4">
            <Badge text="Default" />
            <Badge text="Primary" color=Color::Primary />
            <Badge text="Secondary" color=Color::Secondary />
            <Badge text="Accent" color=Color::Accent />
            <Badge text="Info" color=Color::Info />
            <Badge text="Success" color=Color::Success />
            <Badge text="Warning" color=Color::Warning />
            <Badge text="Error" color=Color::Error />
        </div>
    };

    let size_examples = rsx! {
        <div class="flex flex-wrap items-center gap-4">
            <Badge text="Extra Small" size=Size::Xs />
            <Badge text="Small" size=Size::Sm />
            <Badge text="Medium" size=Size::Md />
            <Badge text="Large" size=Size::Lg />
            <Badge text="Extra Large" size=Size::Xl />
        </div>
    };

    let style_examples = rsx! {
        <div class="flex flex-wrap gap-4">
            <Badge text="Default" />
            <Badge text="Outline" style=BadgeStyle::Outline />
            <Badge text="Dash" style=BadgeStyle::Dash />
            <Badge text="Soft" style=BadgeStyle::Soft />
            <Badge text="Ghost" style=BadgeStyle::Ghost />
        </div>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Badge"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Badges are used to highlight an item's status for quick recognition."
            </p>

            <div class="divider">"Colors"</div>
            {color_examples}

            <div class="divider">"Sizes"</div>
            {size_examples}

            <div class="divider">"Styles"</div>
            {style_examples}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Badge, Color, BadgeStyle};",
                "use rsx_macros::rsx;",
                "",
                "let html = rsx! {",
                "    <Badge text=\"NEW\" color=Color::Success />",
                "    <Badge text=\"Outline\" style=BadgeStyle::Outline />",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/badge").await)
}
