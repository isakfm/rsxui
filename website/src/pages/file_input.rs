use axum::response::Html;
use rsx::rsx;
use rsxui::components::{Color, FileInput, Size};

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <FileInput />
    };

    let color_example = rsx! {
        <div class="flex flex-col gap-4">
            <FileInput color=Color::Primary  />
            <FileInput color=Color::Secondary  />
            <FileInput color=Color::Accent  />
            <FileInput color=Color::Success  />
        </div>
    };

    let size_example = rsx! {
        <div class="flex flex-col gap-4">
            <FileInput size=Size::Xs  />
            <FileInput size=Size::Sm  />
            <FileInput size=Size::Md  />
            <FileInput size=Size::Lg  />
            <FileInput size=Size::Xl  />
        </div>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"File Input"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "File Input is an input field for uploading files."
            </p>

            <div class="divider">"Basic File Input"</div>
            {basic_example}

            <div class="divider">"Colors"</div>
            <p class="text-sm text-gray-600 mb-4">"Different semantic colors."</p>
            {color_example}

            <div class="divider">"Sizes"</div>
            <p class="text-sm text-gray-600 mb-4">"From extra small to extra large."</p>
            {size_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{FileInput, Color, Size};",
                "use rsx::rsx;",
                "",
                "let html = rsx! {",
                "    <FileInput />",
                "};",
                "",
                "// With color",
                "let html = rsx! {",
                "    <FileInput color={Color::Primary} />",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/file-input").await)
}
