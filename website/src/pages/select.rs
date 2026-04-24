use axum::response::Html;
use rsx_macros::rsx;
use rsxui::components::{Color, Select, Size};

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <Select>
            "<option>Option 1</option><option>Option 2</option><option>Option 3</option>"
        </Select>
    };

    let color_example = rsx! {
        <div class="flex flex-col gap-4">
            <Select color=Color::Primary >
                "<option>Primary</option>"
            </Select>
            <Select color=Color::Secondary >
                "<option>Secondary</option>"
            </Select>
            <Select color=Color::Accent >
                "<option>Accent</option>"
            </Select>
        </div>
    };

    let size_example = rsx! {
        <div class="flex flex-col gap-4">
            <Select size=Size::Xs >
                "<option>Extra Small</option>"
            </Select>
            <Select size=Size::Sm >
                "<option>Small</option>"
            </Select>
            <Select size=Size::Md >
                "<option>Medium</option>"
            </Select>
            <Select size=Size::Lg >
                "<option>Large</option>"
            </Select>
        </div>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Select"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Select is a dropdown for choosing from a list of options."
            </p>

            <div class="divider">"Basic Select"</div>
            {basic_example}

            <div class="divider">"Colors"</div>
            <p class="text-sm text-gray-600 mb-4">"Different semantic colors."</p>
            {color_example}

            <div class="divider">"Sizes"</div>
            <p class="text-sm text-gray-600 mb-4">"From extra small to extra large."</p>
            {size_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Select, Color, Size};",
                "use rsx_macros::rsx;",
                "",
                "let html = rsx! {",
                "    <Select>",
                "        \"<option>Option 1</option>\"",
                "    </Select>",
                "};",
                "",
                "// With color",
                "let html = rsx! {",
                "    <Select color={Color::Primary}>",
                "        \"<option>Option 1</option>\"",
                "    </Select>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/select").await)
}
