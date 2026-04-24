use axum::response::Html;
use rsx_macros::rsx;
use rsxui::components::{Kbd, Size};

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <Kbd>"K"</Kbd>
    };

    let sizes_example = rsx! {
        <div class="flex flex-wrap gap-2 items-center">
            <Kbd size=Size::Xs>"XS"</Kbd>
            <Kbd size=Size::Sm>"SM"</Kbd>
            <Kbd size=Size::Md>"MD"</Kbd>
            <Kbd size=Size::Lg>"LG"</Kbd>
            <Kbd size=Size::Xl>"XL"</Kbd>
        </div>
    };

    let shortcut_example = rsx! {
        <div class="flex flex-wrap gap-2">
            <Kbd size=Size::Sm>"Ctrl"</Kbd>
            <span>"+"</span>
            <Kbd size=Size::Sm>"Shift"</Kbd>
            <span>"+"</span>
            <Kbd size=Size::Sm>"T"</Kbd>
        </div>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Kbd"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Kbd is used to display keyboard shortcuts."
            </p>

            <div class="divider">"Basic Kbd"</div>
            {basic_example}

            <div class="divider">"Sizes"</div>
            <p class="text-sm text-gray-600 mb-4">"From extra small to extra large."</p>
            {sizes_example}

            <div class="divider">"Keyboard Shortcut"</div>
            <p class="text-sm text-gray-600 mb-4">"Combine multiple Kbd components for shortcuts."</p>
            {shortcut_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Kbd, Size};",
                "use rsx_macros::rsx;",
                "",
                "let html = rsx! {",
                "    <Kbd size={Size::Lg}>\"Ctrl\"</Kbd>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/kbd").await)
}
