use axum::response::Html;
use rsx_macros::rsx;
use rsxui::components::{
    Collapse, CollapseContent, CollapseModifier, CollapseTitle,
};

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <Collapse>
            <CollapseTitle>"Click to open/close"</CollapseTitle>
            <CollapseContent>"This is the collapsible content."</CollapseContent>
        </Collapse>
    };

    let arrow_example = rsx! {
        <Collapse modifier=CollapseModifier::Arrow>
            <CollapseTitle>"With arrow icon"</CollapseTitle>
            <CollapseContent>"This collapse has an arrow indicator."</CollapseContent>
        </Collapse>
    };

    let plus_example = rsx! {
        <Collapse modifier=CollapseModifier::Plus>
            <CollapseTitle>"With plus icon"</CollapseTitle>
            <CollapseContent>"This collapse has a plus/minus indicator."</CollapseContent>
        </Collapse>
    };

    let open_example = rsx! {
        <Collapse open=true>
            <CollapseTitle>"Open by default"</CollapseTitle>
            <CollapseContent>"This collapse is open by default."</CollapseContent>
        </Collapse>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Collapse"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Collapse is used for showing and hiding content."
            </p>

            <div class="divider">"Basic Collapse"</div>
            <p class="text-sm text-gray-600 mb-4">"Click to toggle content visibility."</p>
            {basic_example}

            <div class="divider">"With Arrow"</div>
            <p class="text-sm text-gray-600 mb-4">"Arrow indicator showing open/close state."</p>
            {arrow_example}

            <div class="divider">"With Plus"</div>
            <p class="text-sm text-gray-600 mb-4">"Plus/minus indicator."</p>
            {plus_example}

            <div class="divider">"Open by Default"</div>
            <p class="text-sm text-gray-600 mb-4">"Collapse that starts open."</p>
            {open_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Collapse, CollapseTitle, CollapseContent};",
                "use rsx_macros::rsx;",
                "",
                "let html = rsx! {",
                "    <Collapse>",
                "        <CollapseTitle>\"Title\"</CollapseTitle>",
                "        <CollapseContent>\"Content\"</CollapseContent>",
                "    </Collapse>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/collapse").await)
}