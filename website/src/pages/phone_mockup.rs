use axum::response::Html;
use rsx::rsx;
use rsxui::components::PhoneMockup;

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <PhoneMockup inner_class="text-white grid place-content-center bg-neutral-90">
            "Hi."
        </PhoneMockup>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Phone Mockup"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Phone mockup shows a mockup of an iPhone."
            </p>

            <div class="divider">"Basic Phone Mockup"</div>
            {basic_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::PhoneMockup;",
                "use rsx::rsx;",
                "",
                "let html = rsx! {",
                "    <PhoneMockup inner_class=\"text-white grid place-content-center bg-neutral-90\">",
                "       \"Hi.\"",
                "    </PhoneMockup>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/phone_mockup").await)
}
