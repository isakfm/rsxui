use axum::response::Html;
use rsxui::components::{Input, InputStyle, Color, Size};
use rsx_macros::rsx;

use crate::layout;

pub async fn page() -> Html<String> {
    let color_examples = rsx! {
        <div class="flex flex-col gap-4 max-w-md">
            <Input placeholder="Default input" />
            <Input placeholder="Primary" color=Color::Primary />
            <Input placeholder="Secondary" color=Color::Secondary />
            <Input placeholder="Accent" color=Color::Accent />
            <Input placeholder="Error" color=Color::Error />
        </div>
    };

    let size_examples = rsx! {
        <div class="flex flex-col gap-4 max-w-md">
            <Input placeholder="Extra Small" size=Size::Xs />
            <Input placeholder="Small" size=Size::Sm />
            <Input placeholder="Medium" size=Size::Md />
            <Input placeholder="Large" size=Size::Lg />
            <Input placeholder="Extra Large" size=Size::Xl />
        </div>
    };

    let style_examples = rsx! {
        <div class="flex flex-col gap-4 max-w-md">
            <Input placeholder="Default style" />
            <Input placeholder="Ghost style" style=InputStyle::Ghost />
        </div>
    };

    let state_examples = rsx! {
        <div class="flex flex-col gap-4 max-w-md">
            <Input placeholder="Disabled" disabled=true />
            <Input placeholder="Required" required=true />
            <Input placeholder="Readonly" readonly=true />
        </div>
    };

    let type_examples = rsx! {
        <div class="flex flex-col gap-4 max-w-md">
            <Input input_type="text" placeholder="Text (default)" />
            <Input input_type="email" placeholder="Email" />
            <Input input_type="password" placeholder="Password" />
            <Input input_type="number" placeholder="Number" />
            <Input input_type="tel" placeholder="Telephone" />
            <Input input_type="url" placeholder="URL" />
        </div>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Input"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Text inputs allow users to enter text into a UI."
            </p>

            <div class="divider">"Colors"</div>
            {color_examples}

            <div class="divider">"Sizes"</div>
            {size_examples}

            <div class="divider">"Styles"</div>
            {style_examples}

            <div class="divider">"States"</div>
            {state_examples}

            <div class="divider">"Input Types"</div>
            {type_examples}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Input, Color, Size};",
                "use rsx_macros::rsx;",
                "",
                "let html = rsx! {",
                "    <Input placeholder=\"Email\" color=Color::Primary />",
                "    <Input input_type=\"password\" placeholder=\"Password\" />",
                "    <Input disabled=true placeholder=\"Disabled\" />",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/input"))
}
