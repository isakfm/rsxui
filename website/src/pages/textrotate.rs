use axum::response::Html;
use rsx_macros::rsx;
use rsxui::components::TextRotate;

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <TextRotate>
            <span>"ONE"</span>
            <span>"TWO"</span>
            <span>"THREE"</span>
        </TextRotate>
    };

    let big_example = rsx! {
        <TextRotate center=true class="text-5xl font-bold">
            <span>"DESIGN"</span>
            <span>"DEVELOP"</span>
            <span>"DEPLOY"</span>
            <span>"SCALE"</span>
            <span>"MAINTAIN"</span>
            <span>"REPEAT"</span>
        </TextRotate>
    };

    let inline_example = rsx! {
        <span>
            "Providing AI Agents for "
            <TextRotate>
                <span class="bg-teal-400 text-teal-800 px-2">"Designers"</span>
                <span class="bg-red-400 text-red-800 px-2">"Developers"</span>
                <span class="bg-blue-400 text-blue-800 px-2">"Managers"</span>
            </TextRotate>
        </span>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Text Rotate"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Text Rotate can show up to 6 lines of text, one at a time, with an infinite loop animation."
            </p>

            <div class="divider">"Basic Text Rotate"</div>
            <p class="text-sm text-gray-600 mb-4">"Rotates through 3 words in 10 seconds."</p>
            {basic_example}

            <div class="divider">"Large Centered Text Rotate"</div>
            <p class="text-sm text-gray-600 mb-4">"Big font size, horizontally centered."</p>
            {big_example}

            <div class="divider">"Inline Text Rotate"</div>
            <p class="text-sm text-gray-600 mb-4">"Different colors for each word inside a sentence."</p>
            {inline_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::TextRotate;",
                "use rsx_macros::rsx;",
                "",
                "let html = rsx! {",
                "    <TextRotate>",
                "        <span>\"ONE\"</span>",
                "        <span>\"TWO\"</span>",
                "        <span>\"THREE\"</span>",
                "    </TextRotate>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/textrotate").await)
}
