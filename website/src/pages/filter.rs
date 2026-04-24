use axum::response::Html;
use rsx_macros::rsx;
use rsxui::components::Filter;

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <Filter tabs=vec!["All".to_string(), "Active".to_string(), "Completed".to_string()] />
    };

    let with_form_example = rsx! {
        <Filter
            tabs=vec!["Tab 1".to_string(), "Tab 2".to_string(), "Tab 3".to_string()]
            use_form=true
        />
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Filter"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Filter is a group of radio buttons for filtering content."
            </p>

            <div class="divider">"Basic Filter"</div>
            {basic_example}

            <div class="divider">"Using Form"</div>
            <p class="text-sm text-gray-600 mb-4">"Filter wrapped in a form element."</p>
            {with_form_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::Filter;",
                "use rsx_macros::rsx;",
                "",
                "let html = rsx! {",
                "    <Filter",
                "        tabs={vec![",
                "            \"All\".to_string(),",
                "            \"Active\".to_string(),",
                "            \"Completed\".to_string(),",
                "        ]}",
                "    />",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/filter").await)
}
