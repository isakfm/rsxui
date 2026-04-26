use axum::response::Html;
use rsx::rsx;
use rsxui::components::{Color, Size, Toggle};

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <Toggle />
    };

    let checked_example = rsx! {
        <Toggle checked=true />
    };

    let color_example = rsx! {
        <div class="flex gap-4 items-center flex-wrap">
            <Toggle color=Color::Primary  checked=true />
            <Toggle color=Color::Secondary  checked=true />
            <Toggle color=Color::Accent  checked=true />
            <Toggle color=Color::Success  checked=true />
            <Toggle color=Color::Warning  checked=true />
            <Toggle color=Color::Error  checked=true />
        </div>
    };

    let size_example = rsx! {
        <div class="flex gap-4 items-center">
            <Toggle size=Size::Xs  checked=true />
            <Toggle size=Size::Sm  checked=true />
            <Toggle size=Size::Md  checked=true />
            <Toggle size=Size::Lg  checked=true />
            <Toggle size=Size::Xl  checked=true />
        </div>
    };

    let disabled_example = rsx! {
        <div class="flex gap-4">
            <Toggle disabled=true />
            <Toggle checked=true disabled=true />
        </div>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Toggle"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Toggle is a checkbox that looks like a switch."
            </p>

            <div class="divider">"Basic Toggle"</div>
            {basic_example}

            <div class="divider">"Checked"</div>
            {checked_example}

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
                "use rsxui::components::{Toggle, Color, Size};",
                "use rsx::rsx;",
                "",
                "let html = rsx! {",
                "    <Toggle />",
                "};",
                "",
                "// With color and checked",
                "let html = rsx! {",
                "    <Toggle color={Color::Primary} checked={true} />",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/toggle").await)
}
