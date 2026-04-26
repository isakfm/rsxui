use axum::response::Html;
use rsx::rsx;
use rsxui::components::Skeleton;

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <Skeleton class="w-32 h-32" />
    };

    let circle_example = rsx! {
        <div class="flex flex-col gap-4 w-52">
            <div class="flex gap-4 items-center">
                <Skeleton class="w-16 h-16 rounded-full shrink-0" />
                <div class="flex flex-col gap-4">
                    <Skeleton class="h-4 w-20" />
                    <Skeleton class="h-4 w-28" />
                </div>
            </div>
            <Skeleton class="h-32 w-full" />
        </div>
    };

    let text_example = rsx! {
        <Skeleton text=true class="text-lg">"AI is thinking harder..."</Skeleton>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Skeleton"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Skeleton is a component that can be used to show a loading state of a component."
            </p>

            <div class="divider">"Basic Skeleton"</div>
            {basic_example}

            <div class="divider">"Complex Layout"</div>
            <p class="text-sm text-gray-600 mb-4">"Combine multiple skeletons for a realistic placeholder."</p>
            {circle_example}

            <div class="divider">"Skeleton Text"</div>
            <p class="text-sm text-gray-600 mb-4">"Animates the text color instead of background."</p>
            {text_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::Skeleton;",
                "use rsx::rsx;",
                "",
                "let html = rsx! {",
                "    <Skeleton class=\"w-32 h-32\" />",
                "};",
                "",
                "// Skeleton text",
                "let html = rsx! {",
                "    <Skeleton text={true} class=\"text-lg\">\"Loading...\"</Skeleton>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/skeleton").await)
}
