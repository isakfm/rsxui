use axum::response::Html;
use rsx::rsx;
use rsxui::components::{Color, Size, Textarea};

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <Textarea placeholder="Type your message here..." />
    };

    let color_example = rsx! {
        <div class="flex flex-col gap-4">
            <Textarea color=Color::Primary  placeholder="Primary" />
            <Textarea color=Color::Secondary  placeholder="Secondary" />
            <Textarea color=Color::Accent  placeholder="Accent" />
        </div>
    };

    let size_example = rsx! {
        <div class="flex flex-col gap-4">
            <Textarea size=Size::Xs  placeholder="Extra Small" />
            <Textarea size=Size::Sm  placeholder="Small" />
            <Textarea size=Size::Md  placeholder="Medium" />
            <Textarea size=Size::Lg  placeholder="Large" />
        </div>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Textarea"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Textarea is a multi-line text input field."
            </p>

            <div class="divider">"Basic Textarea"</div>
            {basic_example}

            <div class="divider">"Colors"</div>
            <p class="text-sm text-gray-600 mb-4">"Different semantic colors."</p>
            {color_example}

            <div class="divider">"Sizes"</div>
            <p class="text-sm text-gray-600 mb-4">"From extra small to extra large."</p>
            {size_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Textarea, Color, Size};",
                "use rsx::rsx;",
                "",
                "let html = rsx! {",
                "    <Textarea placeholder=\"Type here...\" />",
                "};",
                "",
                "// With color",
                "let html = rsx! {",
                "    <Textarea color={Color::Primary} placeholder=\"Primary\" />",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/textarea").await)
}
