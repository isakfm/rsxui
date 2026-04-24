use axum::response::Html;
use rsx_macros::rsx;
use rsxui::components::{
    Color, Stat, StatActions, StatDesc, StatFigure, StatTitle, StatValue, Stats,
};

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <Stats class="shadow">
            <Stat>
                <StatTitle>"Total Page Views"</StatTitle>
                <StatValue>"89,400"</StatValue>
                <StatDesc>"21% more than last month"</StatDesc>
            </Stat>
        </Stats>
    };

    let icons_example = rsx! {
        <Stats class="shadow">
            <Stat>
                <StatFigure color=Color::Primary>
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-8 h-8 stroke-current">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
                    </svg>
                </StatFigure>
                <StatTitle>"Total Likes"</StatTitle>
                <StatValue color=Color::Primary>"25.6K"</StatValue>
                <StatDesc>"21% more than last month"</StatDesc>
            </Stat>
            <Stat>
                <StatFigure color=Color::Secondary>
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-8 h-8 stroke-current">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
                    </svg>
                </StatFigure>
                <StatTitle>"Page Views"</StatTitle>
                <StatValue color=Color::Secondary>"2.6M"</StatValue>
                <StatDesc>"21% more than last month"</StatDesc>
            </Stat>
        </Stats>
    };

    let vertical_example = rsx! {
        <Stats class="shadow" vertical=true>
            <Stat>
                <StatTitle>"Downloads"</StatTitle>
                <StatValue>"31K"</StatValue>
                <StatDesc>"Jan 1st - Feb 1st"</StatDesc>
            </Stat>
            <Stat>
                <StatTitle>"New Users"</StatTitle>
                <StatValue>"4,200"</StatValue>
                <StatDesc>"↗︎ 400 (22%)"</StatDesc>
            </Stat>
            <Stat>
                <StatTitle>"New Registers"</StatTitle>
                <StatValue>"1,200"</StatValue>
                <StatDesc>"↘︎ 90 (14%)"</StatDesc>
            </Stat>
        </Stats>
    };

    let actions_example = rsx! {
        <Stats class="bg-base-100 border border-base-300">
            <Stat>
                <StatTitle>"Account balance"</StatTitle>
                <StatValue>"$89,400"</StatValue>
                <StatActions>
                    <button class="btn btn-xs btn-success">"Add funds"</button>
                </StatActions>
            </Stat>
            <Stat>
                <StatTitle>"Current balance"</StatTitle>
                <StatValue>"$89,400"</StatValue>
                <StatActions>
                    <button class="btn btn-xs">"Withdrawal"</button>
                    <button class="btn btn-xs">"Deposit"</button>
                </StatActions>
            </Stat>
        </Stats>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Stat"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Stat is used to show numbers and data in a block."
            </p>

            <div class="divider">"Basic Stat"</div>
            {basic_example}

            <div class="divider">"Stat with Icons"</div>
            <p class="text-sm text-gray-600 mb-4">"Use StatFigure for icons or images."</p>
            {icons_example}

            <div class="divider">"Vertical Stats"</div>
            <p class="text-sm text-gray-600 mb-4">"Stack stats vertically."</p>
            {vertical_example}

            <div class="divider">"Stat with Actions"</div>
            <p class="text-sm text-gray-600 mb-4">"Add buttons with StatActions."</p>
            {actions_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Stats, Stat, StatTitle, StatValue, StatDesc};",
                "use rsx_macros::rsx;",
                "",
                "let html = rsx! {",
                "    <Stats>",
                "        <Stat>",
                "            <StatTitle>\"Views\"</StatTitle>",
                "            <StatValue>\"89,400\"</StatValue>",
                "            <StatDesc>\"+21%\"</StatDesc>",
                "        </Stat>",
                "    </Stats>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/stat").await)
}
