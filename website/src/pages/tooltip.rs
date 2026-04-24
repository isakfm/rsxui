use axum::response::Html;
use rsx_macros::rsx;
use rsxui::components::{Color, Tooltip, TooltipPlacement};

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <Tooltip data_tip="hello">
            <button class="btn">"Hover me"</button>
        </Tooltip>
    };

    let placements_example = rsx! {
        <div class="flex flex-wrap gap-8">
            <Tooltip data_tip="top" placement=TooltipPlacement::Top>
                <button class="btn">"Top"</button>
            </Tooltip>
            <Tooltip data_tip="bottom" placement=TooltipPlacement::Bottom>
                <button class="btn">"Bottom"</button>
            </Tooltip>
            <Tooltip data_tip="left" placement=TooltipPlacement::Left>
                <button class="btn">"Left"</button>
            </Tooltip>
            <Tooltip data_tip="right" placement=TooltipPlacement::Right>
                <button class="btn">"Right"</button>
            </Tooltip>
        </div>
    };

    let colors_example = rsx! {
        <div class="flex flex-wrap gap-4">
            <Tooltip data_tip="primary" color=Color::Primary>
                <button class="btn">"Primary"</button>
            </Tooltip>
            <Tooltip data_tip="secondary" color=Color::Secondary>
                <button class="btn">"Secondary"</button>
            </Tooltip>
            <Tooltip data_tip="accent" color=Color::Accent>
                <button class="btn">"Accent"</button>
            </Tooltip>
            <Tooltip data_tip="info" color=Color::Info>
                <button class="btn">"Info"</button>
            </Tooltip>
            <Tooltip data_tip="success" color=Color::Success>
                <button class="btn">"Success"</button>
            </Tooltip>
            <Tooltip data_tip="warning" color=Color::Warning>
                <button class="btn">"Warning"</button>
            </Tooltip>
            <Tooltip data_tip="error" color=Color::Error>
                <button class="btn">"Error"</button>
            </Tooltip>
        </div>
    };

    let open_example = rsx! {
        <Tooltip data_tip="Always open" open=true color=Color::Primary>
            <button class="btn">"Open"</button>
        </Tooltip>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Tooltip"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Tooltip can be used to show a message when hovering over an element."
            </p>

            <div class="divider">"Basic Tooltip"</div>
            {basic_example}

            <div class="divider">"Placements"</div>
            <p class="text-sm text-gray-600 mb-4">"Position tooltip on any side."</p>
            {placements_example}

            <div class="divider">"Colors"</div>
            <p class="text-sm text-gray-600 mb-4">"Different semantic colors."</p>
            {colors_example}

            <div class="divider">"Force Open"</div>
            <p class="text-sm text-gray-600 mb-4">"Use tooltip-open to always show."</p>
            {open_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Tooltip, TooltipPlacement, Color};",
                "use rsx_macros::rsx;",
                "",
                "let html = rsx! {",
                "    <Tooltip data_tip=\"hello\" placement=TooltipPlacement::Top color=Color::Primary>",
                "        <button class=\"btn\">\"Hover me\"</button>",
                "    </Tooltip>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/tooltip").await)
}
