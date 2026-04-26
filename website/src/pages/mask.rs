use axum::response::Html;
use rsx::rsx;
use rsxui::components::{Mask, MaskShape};

use crate::layout;

pub async fn page() -> Html<String> {
    let shapes_example = rsx! {
        <div class="flex flex-wrap gap-4 items-center">
            <Mask shape=MaskShape::Squircle class="w-24 h-24">
                <img src="https://img.daisyui.com/images/stock/photo-1567653418876-5bb0e566e1c2.webp" />
            </Mask>
            <Mask shape=MaskShape::Heart class="w-24 h-24">
                <img src="https://img.daisyui.com/images/stock/photo-1567653418876-5bb0e566e1c2.webp" />
            </Mask>
            <Mask shape=MaskShape::Hexagon class="w-24 h-24">
                <img src="https://img.daisyui.com/images/stock/photo-1567653418876-5bb0e566e1c2.webp" />
            </Mask>
            <Mask shape=MaskShape::Circle class="w-24 h-24">
                <img src="https://img.daisyui.com/images/stock/photo-1567653418876-5bb0e566e1c2.webp" />
            </Mask>
            <Mask shape=MaskShape::Star class="w-24 h-24">
                <img src="https://img.daisyui.com/images/stock/photo-1567653418876-5bb0e566e1c2.webp" />
            </Mask>
            <Mask shape=MaskShape::Triangle class="w-24 h-24">
                <img src="https://img.daisyui.com/images/stock/photo-1567653418876-5bb0e566e1c2.webp" />
            </Mask>
        </div>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Mask"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Mask crops the content of the element to common shapes."
            </p>

            <div class="divider">"Shapes"</div>
            <p class="text-sm text-gray-600 mb-4">"Squircle, heart, hexagon, circle, star, triangle."</p>
            {shapes_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Mask, MaskShape};",
                "use rsx::rsx;",
                "",
                "let html = rsx! {",
                "    <Mask shape=MaskShape::Circle class=\"w-24 h-24\">",
                "        <img src=\"photo.jpg\" />",
                "    </Mask>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/mask").await)
}
