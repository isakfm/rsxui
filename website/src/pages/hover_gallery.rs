use axum::response::Html;
use rsx_macros::rsx;
use rsxui::components::HoverGallery;

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <HoverGallery class="max-w-60 rounded-lg overflow-hidden">
            <img src="https://img.daisyui.com/images/stock/daisyui-hat-1.webp" />
            <img src="https://img.daisyui.com/images/stock/daisyui-hat-2.webp" />
            <img src="https://img.daisyui.com/images/stock/daisyui-hat-3.webp" />
            <img src="https://img.daisyui.com/images/stock/daisyui-hat-4.webp" />
        </HoverGallery>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Hover Gallery"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Hover Gallery is a container of images. The first image is visible by default and when you hover horizontally, other images show up."
            </p>

            <div class="divider">"Basic Hover Gallery"</div>
            <p class="text-sm text-gray-600 mb-4">"Hover horizontally over the image to see others."</p>
            {basic_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::HoverGallery;",
                "use rsx_macros::rsx;",
                "",
                "let html = rsx! {",
                "    <HoverGallery class=\"max-w-60\">",
                "        <img src=\"hat-1.webp\" />",
                "        <img src=\"hat-2.webp\" />",
                "        <img src=\"hat-3.webp\" />",
                "        <img src=\"hat-4.webp\" />",
                "    </HoverGallery>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/hover_gallery").await)
}
