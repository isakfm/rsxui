use axum::response::Html;
use rsx::rsx;
use rsxui::components::WindowMockup;

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <WindowMockup class="border">
            <div class="flex justify-center px-4 py-16 bg-base-200">"Hello World!"</div>
        </WindowMockup>
    };

    let with_title_example = rsx! {
        <WindowMockup>
            <div class="bg-base-200 flex justify-center px-4 py-16">
                <div class="text-center">
                    <h2 class="text-xl font-bold">"Window Title"</h2>
                    <p>"Content goes here."</p>
                </div>
            </div>
        </WindowMockup>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Window Mockup"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Window mockup shows a box that looks like an operating system window."
            </p>

            <div class="divider">"Basic Window Mockup"</div>
            {basic_example}

            <div class="divider">"With Content"</div>
            <p class="text-sm text-gray-600 mb-4">"Add any content inside."</p>
            {with_title_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::WindowMockup;",
                "use rsx::rsx;",
                "",
                "let html = rsx! {",
                "    <WindowMockup>",
                "        <div class=\"flex justify-center px-4 py-16 bg-base-200\">\"Hello World!\"</div>",
                "    </WindowMockup>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/window_mockup").await)
}
