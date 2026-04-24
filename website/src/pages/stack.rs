use axum::response::Html;
use rsx_macros::rsx;
use rsxui::components::{Stack, StackModifier};

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <Stack class="w-32">
            <img src="https://img.daisyui.com/images/stock/photo-1559703248-dcaaec9fab78.webp" class="rounded" />
            <img src="https://img.daisyui.com/images/stock/photo-1565098772267-60af42b81ef2.webp" class="rounded" />
            <img src="https://img.daisyui.com/images/stock/photo-1572635148818-ef6fd45eb394.webp" class="rounded" />
        </Stack>
    };

    let modifiers_example = rsx! {
        <div class="flex flex-wrap gap-8">
            <Stack modifier=StackModifier::Top class="w-32">
                <img src="https://img.daisyui.com/images/stock/photo-1559703248-dcaaec9fab78.webp" class="rounded" />
                <img src="https://img.daisyui.com/images/stock/photo-1565098772267-60af42b81ef2.webp" class="rounded" />
                <img src="https://img.daisyui.com/images/stock/photo-1572635148818-ef6fd45eb394.webp" class="rounded" />
            </Stack>
            <Stack modifier=StackModifier::End class="w-32">
                <img src="https://img.daisyui.com/images/stock/photo-1559703248-dcaaec9fab78.webp" class="rounded" />
                <img src="https://img.daisyui.com/images/stock/photo-1565098772267-60af42b81ef2.webp" class="rounded" />
                <img src="https://img.daisyui.com/images/stock/photo-1572635148818-ef6fd45eb394.webp" class="rounded" />
            </Stack>
        </div>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Stack"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Stack visually puts elements on top of each other."
            </p>

            <div class="divider">"Basic Stack"</div>
            {basic_example}

            <div class="divider">"Modifiers"</div>
            <p class="text-sm text-gray-600 mb-4">"Stack-top, stack-bottom, stack-start, stack-end."</p>
            {modifiers_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Stack, StackModifier};",
                "use rsx_macros::rsx;",
                "",
                "let html = rsx! {",
                "    <Stack modifier=StackModifier::Top class=\"w-32\">",
                "        <img src=\"1.jpg\" />",
                "        <img src=\"2.jpg\" />",
                "        <img src=\"3.jpg\" />",
                "    </Stack>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/stack").await)
}
