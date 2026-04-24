use axum::response::Html;
use rsx_macros::rsx;
use rsxui::components::Join;

use crate::layout;

pub async fn page() -> Html<String> {
    let buttons_example = rsx! {
        <Join>
            <button class="btn join-item">"Button"</button>
            <button class="btn join-item">"Button"</button>
            <button class="btn join-item">"Button"</button>
        </Join>
    };

    let vertical_example = rsx! {
        <Join vertical=true>
            <button class="btn join-item">"Vertical 1"</button>
            <button class="btn join-item">"Vertical 2"</button>
            <button class="btn join-item">"Vertical 3"</button>
        </Join>
    };

    let inputs_example = rsx! {
        <Join>
            <input class="input join-item" placeholder="Email" />
            <button class="btn btn-primary join-item">"Subscribe"</button>
        </Join>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Join"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Join is a container for grouping multiple items. It applies border radius to the first and last item."
            </p>

            <div class="divider">"Button Join"</div>
            <p class="text-sm text-gray-600 mb-4">"Group buttons together."</p>
            {buttons_example}

            <div class="divider">"Vertical Join"</div>
            <p class="text-sm text-gray-600 mb-4">"Stack items vertically."</p>
            {vertical_example}

            <div class="divider">"Input + Button Join"</div>
            <p class="text-sm text-gray-600 mb-4">"Combine inputs and buttons."</p>
            {inputs_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::Join;",
                "use rsx_macros::rsx;",
                "",
                "let html = rsx! {",
                "    <Join>",
                "        <button class=\"btn join-item\">\"One\"</button>",
                "        <button class=\"btn join-item\">\"Two\"</button>",
                "    </Join>",
                "};",
                "",
                "// Vertical",
                "let html = rsx! {",
                "    <Join vertical={true}>",
                "        <button class=\"btn join-item\">\"A\"</button>",
                "        <button class=\"btn join-item\">\"B\"</button>",
                "    </Join>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/join").await)
}
