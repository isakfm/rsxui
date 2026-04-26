use axum::response::Html;
use rsx::rsx;
use rsxui::components::RadialProgress;

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <RadialProgress value="70">"70%"</RadialProgress>
    };

    let sizes_example = rsx! {
        <div class="flex flex-wrap gap-4 items-center">
            <RadialProgress value="80" size="3rem">"80%"</RadialProgress>
            <RadialProgress value="60" size="5rem">"60%"</RadialProgress>
            <RadialProgress value="40" size="7rem">"40%"</RadialProgress>
        </div>
    };

    let thickness_example = rsx! {
        <div class="flex flex-wrap gap-4 items-center">
            <RadialProgress value="50" thickness="2px" size="5rem">"50%"</RadialProgress>
            <RadialProgress value="50" thickness="8px" size="5rem">"50%"</RadialProgress>
        </div>
    };

    let colors_example = rsx! {
        <div class="flex flex-wrap gap-4 items-center">
            <RadialProgress value="90" class="text-primary" size="5rem">"90%"</RadialProgress>
            <RadialProgress value="75" class="text-secondary" size="5rem">"75%"</RadialProgress>
            <RadialProgress value="60" class="text-accent" size="5rem">"60%"</RadialProgress>
        </div>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Radial Progress"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Radial progress can be used to show the progress of a task or to show the passing of time."
            </p>

            <div class="divider">"Basic Radial Progress"</div>
            {basic_example}

            <div class="divider">"Sizes"</div>
            <p class="text-sm text-gray-600 mb-4">"Use --size CSS variable for custom sizes."</p>
            {sizes_example}

            <div class="divider">"Thickness"</div>
            <p class="text-sm text-gray-600 mb-4">"Use --thickness CSS variable for custom thickness."</p>
            {thickness_example}

            <div class="divider">"Colors"</div>
            <p class="text-sm text-gray-600 mb-4">"Use Tailwind text color utilities."</p>
            {colors_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::RadialProgress;",
                "use rsx::rsx;",
                "",
                "let html = rsx! {",
                "    <RadialProgress value=\"70\" size=\"5rem\" thickness=\"4px\">",
                "        \"70%\"",
                "    </RadialProgress>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/radial_progress").await)
}
