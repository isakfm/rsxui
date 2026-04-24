use axum::response::Html;
use rsx_macros::rsx;
use rsxui::components::CodeMockup;

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <CodeMockup>
            <pre data-prefix="$"><code>"npm i daisyui"</code></pre>
            <pre data-prefix=">" class="text-warning"><code>"installing..."</code></pre>
            <pre data-prefix=">" class="text-success"><code>"done!"</code></pre>
        </CodeMockup>
    };

    let with_color_example = rsx! {
        <CodeMockup class="bg-primary text-primary-content">
            <pre data-prefix="~"><code>"cargo add rsxui"</code></pre>
        </CodeMockup>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Code Mockup"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Code mockup is used to show a block of code in a box that looks like a code editor."
            </p>

            <div class="divider">"Basic Code Mockup"</div>
            {basic_example}

            <div class="divider">"With Color"</div>
            <p class="text-sm text-gray-600 mb-4">"Use Tailwind utilities for custom colors."</p>
            {with_color_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::CodeMockup;",
                "use rsx_macros::rsx;",
                "",
                "let html = rsx! {",
                "    <CodeMockup>",
                "        <pre data-prefix=\"$\"><code>\"npm i daisyui\"</code></pre>",
                "        <pre data-prefix=\">\" class=\"text-warning\"><code>\"installing...\"</code></pre>",
                "    </CodeMockup>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/code_mockup").await)
}
