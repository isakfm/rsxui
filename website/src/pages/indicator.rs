use axum::response::Html;
use rsx_macros::rsx;
use rsxui::components::{Indicator, IndicatorItem};

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <Indicator>
            <IndicatorItem>
                <span class="badge badge-primary">"New"</span>
            </IndicatorItem>
            <div class="bg-base-300 grid h-32 w-32 place-items-center rounded-box">"Content"</div>
        </Indicator>
    };

    let badge_example = rsx! {
        <Indicator>
            <IndicatorItem end_=true top=true>
                <div class="badge badge-secondary">"99+"</div>
            </IndicatorItem>
            <div class="bg-base-300 grid h-32 w-32 place-items-center rounded-box">
                <span class="text-3xl">"📫"</span>
            </div>
        </Indicator>
    };

    let avatar_example = rsx! {
        <Indicator>
            <IndicatorItem end_=true bottom=true>
                <span class="badge badge-success badge-sm">"online"</span>
            </IndicatorItem>
            <div class="bg-base-300 grid h-32 w-32 place-items-center rounded-box">
                <span class="text-3xl">"👤"</span>
            </div>
        </Indicator>
    };

    let button_example = rsx! {
        <Indicator>
            <IndicatorItem start=true top=true>
                <button class="btn btn-circle btn-xs btn-error">"X"</button>
            </IndicatorItem>
            <div class="bg-base-300 grid h-32 w-32 place-items-center rounded-box">"Card"</div>
        </Indicator>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Indicator"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Indicators are used to place an element on the corner of another element."
            </p>

            <div class="divider">"Basic Indicator"</div>
            {basic_example}

            <div class="divider">"Badge Indicator"</div>
            <p class="text-sm text-gray-600 mb-4">"Badge in top-end corner."</p>
            {badge_example}

            <div class="divider">"Status Indicator"</div>
            <p class="text-sm text-gray-600 mb-4">"Status badge in bottom-end corner."</p>
            {avatar_example}

            <div class="divider">"Button Indicator"</div>
            <p class="text-sm text-gray-600 mb-4">"Action button in top-start corner."</p>
            {button_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Indicator, IndicatorItem};",
                "use rsx_macros::rsx;",
                "",
                "let html = rsx! {",
                "    <Indicator>",
                "        <IndicatorItem end_={true} top={true}>",
                "            <span class=\"badge badge-primary\">\"New\"</span>",
                "        </IndicatorItem>",
                "        <div>\"Content\"</div>",
                "    </Indicator>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/indicator").await)
}
