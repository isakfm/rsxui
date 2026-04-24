use axum::response::Html;
use rsx_macros::rsx;
use rsxui::components::{FloatingLabel, Label};

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <Label text="Email Address">
            "<input type=\"email\" placeholder=\"you@example.com\" class=\"input\" />"
        </Label>
    };

    let floating_example = rsx! {
        <FloatingLabel text="Username">
            "<input type=\"text\" placeholder=\"johndoe\" class=\"input\" />"
        </FloatingLabel>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Label"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Label provides a name or title for an input field."
            </p>

            <div class="divider">"Basic Label"</div>
            {basic_example}

            <div class="divider">"Floating Label"</div>
            <p class="text-sm text-gray-600 mb-4">"Label that floats above the input when focused."</p>
            {floating_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Label, FloatingLabel};",
                "use rsx_macros::rsx;",
                "",
                "// Regular label",
                "let html = rsx! {",
                "    <Label text=\"Email\">",
                "        \"<input type=\\\"email\\\" class=\\\"input\\\" />\"",
                "    </Label>",
                "};",
                "",
                "// Floating label",
                "let html = rsx! {",
                "    <FloatingLabel text=\"Username\">",
                "        \"<input type=\\\"text\\\" class=\\\"input\\\" />\"",
                "    </FloatingLabel>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/label").await)
}
