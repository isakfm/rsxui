use axum::response::Html;
use rsx::rsx;
use rsxui::components::Hover3d;

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <Hover3d class="my-12 mx-2">
            <figure class="max-w-100 rounded-2xl">
                <img src="https://img.daisyui.com/images/stock/creditcard.webp" alt="Tailwind CSS 3D card" />
            </figure>
        </Hover3d>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Hover 3D"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Hover 3D is a wrapper component that adds a 3D hover effect to its content. When you hover over the component, it tilts and rotates based on the mouse position."
            </p>

            <div class="divider">"Basic Hover 3D"</div>
            <p class="text-sm text-gray-600 mb-4">"Hover over the image to see the 3D effect."</p>
            {basic_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::Hover3d;",
                "use rsx::rsx;",
                "",
                "let html = rsx! {",
                "    <Hover3d class=\"my-12 mx-2\">",
                "        <figure class=\"max-w-100 rounded-2xl\">",
                "            <img src=\"card.jpg\" alt=\"Card\" />",
                "        </figure>",
                "    </Hover3d>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/hover3d").await)
}
