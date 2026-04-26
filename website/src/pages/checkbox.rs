use axum::response::Html;
use rsx::rsx;
use rsxui::components::{Checkbox, Color, Size};

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <Checkbox />
    };

    let checked_example = rsx! {
        <Checkbox checked=true />
    };

    let color_example = rsx! {
        <div class="flex gap-4 items-center flex-wrap">
            <Checkbox color=Color::Primary  checked=true />
            <Checkbox color=Color::Secondary  checked=true />
            <Checkbox color=Color::Accent  checked=true />
            <Checkbox color=Color::Success  checked=true />
            <Checkbox color=Color::Warning  checked=true />
            <Checkbox color=Color::Error  checked=true />
            <Checkbox color=Color::Info  checked=true />
        </div>
    };

    let size_example = rsx! {
        <div class="flex gap-4 items-center">
            <Checkbox size=Size::Xs  checked=true />
            <Checkbox size=Size::Sm  checked=true />
            <Checkbox size=Size::Md  checked=true />
            <Checkbox size=Size::Lg  checked=true />
            <Checkbox size=Size::Xl  checked=true />
        </div>
    };

    let disabled_example = rsx! {
        <div class="flex gap-4">
            <Checkbox disabled=true />
            <Checkbox checked=true disabled=true />
        </div>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Checkbox"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Checkbox is used to select or deselect a value."
            </p>

            <div class="divider">"Basic Checkbox"</div>
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
                "use rsxui::components::{Checkbox, Color, Size};",
                "use rsx::rsx;",
                "",
                "let html = rsx! {",
                "    <Checkbox />",
                "};",
                "",
                "// With color and checked",
                "let html = rsx! {",
                "    <Checkbox color={Color::Primary} checked={true} />",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/checkbox").await)
}
