use axum::response::Html;
use rsx::rsx;
use rsxui::components::{Color, Radio, Size};

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <Radio name="radio-basic" />
    };

    let checked_example = rsx! {
        <Radio name="radio-checked" checked=true />
    };

    let color_example = rsx! {
        <div class="flex gap-4 items-center flex-wrap">
            <Radio name="radio-color" color=Color::Primary  checked=true />
            <Radio name="radio-color" color=Color::Secondary  />
            <Radio name="radio-color" color=Color::Accent  />
            <Radio name="radio-color" color=Color::Success  />
            <Radio name="radio-color" color=Color::Warning  />
            <Radio name="radio-color" color=Color::Error  />
        </div>
    };

    let size_example = rsx! {
        <div class="flex gap-4 items-center">
            <Radio name="radio-size" size=Size::Xs  />
            <Radio name="radio-size" size=Size::Sm  />
            <Radio name="radio-size" size=Size::Md  checked=true />
            <Radio name="radio-size" size=Size::Lg  />
            <Radio name="radio-size" size=Size::Xl  />
        </div>
    };

    let disabled_example = rsx! {
        <div class="flex gap-4">
            <Radio name="radio-disabled" disabled=true />
            <Radio name="radio-disabled" checked=true disabled=true />
        </div>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Radio"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Radio buttons are used to select a single option from a list."
            </p>

            <div class="divider">"Basic Radio"</div>
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
                "use rsxui::components::{Radio, Color, Size};",
                "use rsx::rsx;",
                "",
                "let html = rsx! {",
                "    <Radio name=\"my-radio\" />",
                "};",
                "",
                "// With color and checked",
                "let html = rsx! {",
                "    <Radio name=\"my-radio\" color={Color::Primary} checked={true} />",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/radio").await)
}
