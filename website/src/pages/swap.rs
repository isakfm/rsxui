use axum::response::Html;
use rsx_macros::rsx;
use rsxui::components::{Swap, SwapOff, SwapOn};

use crate::layout;

pub async fn page() -> Html<String> {
    let text_example = rsx! {
        <Swap>
            <input type="checkbox" />
            <SwapOn>"ON"</SwapOn>
            <SwapOff>"OFF"</SwapOff>
        </Swap>
    };

    let rotate_example = rsx! {
        <Swap rotate=true class="text-5xl">
            <input type="checkbox" />
            <SwapOn>"😈"</SwapOn>
            <SwapOff>"😇"</SwapOff>
        </Swap>
    };

    let flip_example = rsx! {
        <Swap flip=true class="text-5xl">
            <input type="checkbox" />
            <SwapOn>"🌙"</SwapOn>
            <SwapOff>"☀️"</SwapOff>
        </Swap>
    };

    let active_example = rsx! {
        <div class="flex gap-8">
            <Swap class="text-5xl">
                <SwapOn>"🥵"</SwapOn>
                <SwapOff>"🥶"</SwapOff>
            </Swap>
            <Swap active=true class="text-5xl">
                <SwapOn>"🥳"</SwapOn>
                <SwapOff>"😭"</SwapOff>
            </Swap>
        </div>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Swap"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Swap allows you to toggle the visibility of two elements using a checkbox or a class name."
            </p>

            <div class="divider">"Text Swap"</div>
            {text_example}

            <div class="divider">"Rotate Effect"</div>
            <p class="text-sm text-gray-600 mb-4">"Adds rotate animation to swap."</p>
            {rotate_example}

            <div class="divider">"Flip Effect"</div>
            <p class="text-sm text-gray-600 mb-4">"Adds flip animation to swap."</p>
            {flip_example}

            <div class="divider">"Active State"</div>
            <p class="text-sm text-gray-600 mb-4">"Use swap-active class instead of checkbox."</p>
            {active_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Swap, SwapOn, SwapOff};",
                "use rsx_macros::rsx;",
                "",
                "let html = rsx! {",
                "    <Swap rotate={true}>",
                "        <input type=\"checkbox\" />",
                "        <SwapOn>\"😈\"</SwapOn>",
                "        <SwapOff>\"😇\"</SwapOff>",
                "    </Swap>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/swap").await)
}
