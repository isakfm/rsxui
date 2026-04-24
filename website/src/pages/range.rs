use axum::response::Html;
use rsx_macros::rsx;
use rsxui::components::{Color, Range, Size};

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <Range />
    };

    let value_example = rsx! {
        <Range value="50" min="0" max="100" />
    };

    let color_example = rsx! {
        <div class="flex flex-col gap-4">
            <Range color=Color::Primary  value="20" />
            <Range color=Color::Secondary  value="40" />
            <Range color=Color::Accent  value="60" />
            <Range color=Color::Success  value="80" />
        </div>
    };

    let size_example = rsx! {
        <div class="flex flex-col gap-4">
            <Range size=Size::Xs  value="20" />
            <Range size=Size::Sm  value="40" />
            <Range size=Size::Md  value="60" />
            <Range size=Size::Lg  value="80" />
            <Range size=Size::Xl  value="100" />
        </div>
    };

    let disabled_example = rsx! {
        <Range value="50" disabled=true />
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Range"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Range slider for selecting a numeric value."
            </p>

            <div class="divider">"Basic Range"</div>
            {basic_example}

            <div class="divider">"With Value"</div>
            <p class="text-sm text-gray-600 mb-4">"Set min, max, and value."</p>
            {value_example}

            <div class="divider">"Colors"</div>
            <p class="text-sm text-gray-600 mb-4">"Different semantic colors."</p>
            {color_example}

            <div class="divider">"Sizes"</div>
            <p class="text-sm text-gray-600 mb-4">"From extra small to extra large."</p>
            {size_example}

            <div class="divider">"Disabled"</div>
            <p class="text-sm text-gray-600 mb-4">"Disabled state."</p>
            {disabled_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Range, Color, Size};",
                "use rsx_macros::rsx;",
                "",
                "let html = rsx! {",
                "    <Range />",
                "};",
                "",
                "// With value range",
                "let html = rsx! {",
                "    <Range value={50} min={0} max={100} />",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/range").await)
}
