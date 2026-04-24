use axum::response::Html;
use rsx_macros::rsx;
use rsxui::components::Breadcrumb;

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <Breadcrumb>
            <li><a>"Home"</a></li>
            <li><a>"Documents"</a></li>
            <li>"Add Document"</li>
        </Breadcrumb>
    };

    let with_icons_example = rsx! {
        <Breadcrumb>
            <li>
                <a>
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="w-4 h-4 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"></path></svg>
                    "Home"
                </a>
            </li>
            <li>
                <a>
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="w-4 h-4 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"></path></svg>
                    "Documents"
                </a>
            </li>
            <li>
                <span class="inline-flex items-center gap-2">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="w-4 h-4 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 13h6m-3-3v6m5 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path></svg>
                    "Add Document"
                </span>
            </li>
        </Breadcrumb>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Breadcrumb"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Breadcrumbs helps users to navigate."
            </p>

            <div class="divider">"Basic Breadcrumb"</div>
            {basic_example}

            <div class="divider">"With Icons"</div>
            <p class="text-sm text-gray-600 mb-4">"Add icons inside the links."</p>
            {with_icons_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::Breadcrumb;",
                "use rsx_macros::rsx;",
                "",
                "let html = rsx! {",
                "    <Breadcrumb>",
                "        <li><a>\"Home\"</a></li>",
                "        <li><a>\"Documents\"</a></li>",
                "        <li>\"Add Document\"</li>",
                "    </Breadcrumb>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/breadcrumb").await)
}
