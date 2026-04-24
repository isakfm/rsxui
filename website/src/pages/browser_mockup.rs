use axum::response::Html;
use rsx_macros::rsx;
use rsxui::components::{BrowserMockup, BrowserToolbar};

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <BrowserMockup class="border">
            <BrowserToolbar>
                <div class="input">"https://daisyui.com"</div>
            </BrowserToolbar>
            <div class="flex justify-center px-4 py-16 bg-base-200">"Hello!"</div>
        </BrowserMockup>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Browser Mockup"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Browser mockup shows a box that looks like a browser window."
            </p>

            <div class="divider">"Basic Browser Mockup"</div>
            {basic_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{BrowserMockup, BrowserToolbar};",
                "use rsx_macros::rsx;",
                "",
                "let html = rsx! {",
                "    <BrowserMockup>",
                "        <BrowserToolbar>",
                "            <div class=\"input\">\"https://daisyui.com\"</div>",
                "        </BrowserToolbar>",
                "        <div>\"Hello!\"</div>",
                "    </BrowserMockup>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/browser_mockup").await)
}
