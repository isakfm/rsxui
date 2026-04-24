use axum::response::Html;
use rsx_macros::rsx;
use rsxui::components::Fieldset;

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <Fieldset title="Personal Information">
            "<input type=\"text\" placeholder=\"Name\" class=\"input\" />"
        </Fieldset>
    };

    let with_description_example = rsx! {
        <Fieldset title="Email Address" description="We will never share your email with anyone.">
            "<input type=\"email\" placeholder=\"you@example.com\" class=\"input\" />"
        </Fieldset>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Fieldset"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Fieldset is a container for grouping related form elements."
            </p>

            <div class="divider">"Basic Fieldset"</div>
            {basic_example}

            <div class="divider">"With Description"</div>
            <p class="text-sm text-gray-600 mb-4">"Add a description below the content."</p>
            {with_description_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::Fieldset;",
                "use rsx_macros::rsx;",
                "",
                "let html = rsx! {",
                "    <Fieldset title=\"Personal Information\">",
                "        \"<input type=\\\"text\\\" />\"",
                "    </Fieldset>",
                "};",
                "",
                "// With description",
                "let html = rsx! {",
                "    <Fieldset",
                "        title=\"Email\"",
                "        description=\"We will never share your email.\"",
                "    >",
                "        \"<input type=\\\"email\\\" />\"",
                "    </Fieldset>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/fieldset").await)
}
