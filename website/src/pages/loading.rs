use axum::response::Html;
use rsx::rsx;
use rsxui::components::{Loading, LoadingStyle, Size};

use crate::layout;

pub async fn page() -> Html<String> {
    let styles_example = rsx! {
        <div class="flex flex-wrap gap-4 items-center">
            <Loading style=LoadingStyle::Spinner />
            <Loading style=LoadingStyle::Dots />
            <Loading style=LoadingStyle::Ring />
            <Loading style=LoadingStyle::Ball />
            <Loading style=LoadingStyle::Bars />
            <Loading style=LoadingStyle::Infinity />
        </div>
    };

    let sizes_example = rsx! {
        <div class="flex flex-wrap gap-4 items-center">
            <Loading size=Size::Xs />
            <Loading size=Size::Sm />
            <Loading size=Size::Md />
            <Loading size=Size::Lg />
            <Loading size=Size::Xl />
        </div>
    };

    let colors_example = rsx! {
        <div class="flex flex-wrap gap-4 items-center">
            <Loading class="text-primary" />
            <Loading class="text-secondary" />
            <Loading class="text-accent" />
            <Loading class="text-success" />
            <Loading class="text-warning" />
            <Loading class="text-error" />
        </div>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Loading"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Loading shows an animation to indicate that something is loading."
            </p>

            <div class="divider">"Styles"</div>
            <p class="text-sm text-gray-600 mb-4">"Different animation styles."</p>
            {styles_example}

            <div class="divider">"Sizes"</div>
            <p class="text-sm text-gray-600 mb-4">"From extra small to extra large."</p>
            {sizes_example}

            <div class="divider">"Colors"</div>
            <p class="text-sm text-gray-600 mb-4">"Use Tailwind text color utilities."</p>
            {colors_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Loading, LoadingStyle, Size};",
                "use rsx::rsx;",
                "",
                "let html = rsx! {",
                "    <Loading style={LoadingStyle::Spinner} size={Size::Lg} />",
                "};",
                "",
                "// With color",
                "let html = rsx! {",
                "    <Loading class=\"text-primary\" />",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/loading").await)
}
