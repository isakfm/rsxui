use axum::response::Html;
use rsx::rsx;
use rsxui::components::{Diff, DiffItem1, DiffItem2, DiffResizer};

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <Diff class="aspect-16/9 max-w-md">
            <DiffItem1>
                <img src="https://img.daisyui.com/images/stock/photo-1560717789-0ac7c58ac90a.webp" alt="Before" />
            </DiffItem1>
            <DiffItem2>
                <img src="https://img.daisyui.com/images/stock/photo-1560717789-0ac7c58ac90a-blur.webp" alt="After" />
            </DiffItem2>
            <DiffResizer />
        </Diff>
    };

    let text_example = rsx! {
        <Diff class="aspect-16/9 max-w-md">
            <DiffItem1>
                <div class="bg-primary text-primary-content grid place-content-center text-9xl font-black">
                    "DAISY"
                </div>
            </DiffItem1>
            <DiffItem2>
                <div class="bg-base-200 grid place-content-center text-9xl font-black">
                    "DAISY"
                </div>
            </DiffItem2>
            <DiffResizer />
        </Diff>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Diff"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Diff shows a side-by-side comparison of two items with a draggable slider."
            </p>

            <div class="divider">"Basic Diff"</div>
            <p class="text-sm text-gray-600 mb-4">"Drag the slider to compare before and after."</p>
            {basic_example}

            <div class="divider">"Text Diff"</div>
            <p class="text-sm text-gray-600 mb-4">"Compare styled text content."</p>
            {text_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Diff, DiffItem1, DiffItem2, DiffResizer};",
                "use rsx::rsx;",
                "",
                "let html = rsx! {",
                "    <Diff class=\"aspect-16/9\">",
                "        <DiffItem1>",
                "            <img src=\"before.jpg\" alt=\"Before\" />",
                "        </DiffItem1>",
                "        <DiffItem2>",
                "            <img src=\"after.jpg\" alt=\"After\" />",
                "        </DiffItem2>",
                "        <DiffResizer />",
                "    </Diff>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/diff").await)
}
