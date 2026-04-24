use axum::response::Html;
use rsx_macros::rsx;
use rsxui::components::{Toast, ToastHorizontal, ToastVertical};

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <Toast>
            <div class="alert alert-info">
                <span>"New registration."</span>
            </div>
        </Toast>
    };

    let multiple_example = rsx! {
        <Toast horizontal=ToastHorizontal::Center>
            <div class="alert alert-info">
                <span>"New email arrived."</span>
            </div>
            <div class="alert alert-success">
                <span>"Message sent successfully."</span>
            </div>
        </Toast>
    };

    let placement_example = rsx! {
        <div class="relative h-48 bg-base-200 rounded-box overflow-hidden">
            <Toast horizontal=ToastHorizontal::End vertical=ToastVertical::Top class="absolute">
                <div class="alert alert-warning">
                    <span>"Top right toast"</span>
                </div>
            </Toast>
        </div>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Toast"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Toast is a wrapper to stack elements, positioned on the corner of page."
            </p>

            <div class="divider">"Basic Toast"</div>
            {basic_example}

            <div class="divider">"Multiple Toasts"</div>
            <p class="text-sm text-gray-600 mb-4">"Stack multiple alerts inside a toast."</p>
            {multiple_example}

            <div class="divider">"Placement"</div>
            <p class="text-sm text-gray-600 mb-4">"Position toast on any corner."</p>
            {placement_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Toast, ToastHorizontal, ToastVertical};",
                "use rsx_macros::rsx;",
                "",
                "let html = rsx! {",
                "    <Toast horizontal={ToastHorizontal::End} vertical={ToastVertical::Top}>",
                "        <div class=\"alert alert-info\">",
                "            <span>\"New mail arrived.\"</span>",
                "        </div>",
                "    </Toast>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/toast").await)
}
