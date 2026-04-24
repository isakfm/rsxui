use axum::response::Html;
use rsx_macros::rsx;
use rsxui::components::{Countdown, CountdownGroup, CountdownValue};

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <Countdown value=42 />
    };

    let large_example = rsx! {
        <Countdown value=60 class="text-5xl font-mono" />
    };

    let digits_example = rsx! {
        <Countdown value=5 class="font-mono text-6xl" digits=2 />
    };

    let dynamic_example = rsx! {
        <Countdown value=30 dynamic=true class="font-mono text-6xl" />
    };

    let timer_example = rsx! {
        <div class="grid grid-flow-col gap-5 text-center auto-cols-max">
            <div class="flex flex-col">
                <Countdown value=15 class="text-5xl font-mono" />
                <span class="text-sm">"days"</span>
            </div>
            <div class="flex flex-col">
                <Countdown value=10 class="text-5xl font-mono" />
                <span class="text-sm">"hours"</span>
            </div>
            <div class="flex flex-col">
                <Countdown value=24 class="text-5xl font-mono" />
                <span class="text-sm">"min"</span>
            </div>
            <div class="flex flex-col">
                <Countdown value=59 class="text-5xl font-mono" />
                <span class="text-sm">"sec"</span>
            </div>
        </div>
    };

    let clock_example = rsx! {
        <CountdownGroup class="font-mono text-2xl">
            <CountdownValue value=10 />
            "h "
            <CountdownValue value=24 />
            "m "
            <CountdownValue value=59 />
            "s"
        </CountdownGroup>
    };

    let colon_example = rsx! {
        <CountdownGroup class="font-mono text-2xl">
            <CountdownValue value=10 />
            ":"
            <CountdownValue value=24 digits=2 />
            ":"
            <CountdownValue value=59 digits=2 />
        </CountdownGroup>
    };

    let boxed_example = rsx! {
        <div class="grid grid-flow-col gap-5 text-center auto-cols-max">
            <div class="flex flex-col p-2 bg-neutral rounded-box text-neutral-content">
                <Countdown value=15 class="font-mono text-5xl" />
                <span>"days"</span>
            </div>
            <div class="flex flex-col p-2 bg-neutral rounded-box text-neutral-content">
                <Countdown value=10 class="font-mono text-5xl" />
                <span>"hours"</span>
            </div>
            <div class="flex flex-col p-2 bg-neutral rounded-box text-neutral-content">
                <Countdown value=24 class="font-mono text-5xl" />
                <span>"min"</span>
            </div>
            <div class="flex flex-col p-2 bg-neutral rounded-box text-neutral-content">
                <Countdown value=59 class="font-mono text-5xl" />
                <span>"sec"</span>
            </div>
        </div>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Countdown"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Countdown gives you a transition effect when you change a number between 0 to 999."
            </p>

            <div class="divider">"Basic Countdown"</div>
            <p class="text-sm text-gray-600 mb-4">"Simple countdown with a number."</p>
            {basic_example}

            <div class="divider">"Large Countdown"</div>
            <p class="text-sm text-gray-600 mb-4">"Bigger text with monospace font."</p>
            {large_example}

            <div class="divider">"With 2 Digits"</div>
            <p class="text-sm text-gray-600 mb-4">"Minimum 2 digits display using digits prop."</p>
            {digits_example}

            <div class="divider">"Dynamic Countdown"</div>
            <p class="text-sm text-gray-600 mb-4">"Auto-decrementing countdown with JavaScript. Starts at 30."</p>
            {dynamic_example}

            <div class="divider">"Timer Layout"</div>
            <p class="text-sm text-gray-600 mb-4">"Days, hours, minutes, and seconds."</p>
            {timer_example}

            <div class="divider">"Clock Countdown"</div>
            <p class="text-sm text-gray-600 mb-4">"Grouped countdown with inline labels using CountdownGroup and CountdownValue."</p>
            {clock_example}

            <div class="divider">"With Colons"</div>
            <p class="text-sm text-gray-600 mb-4">"Time format with colon separators and 2-digit display."</p>
            {colon_example}

            <div class="divider">"In Boxes"</div>
            <p class="text-sm text-gray-600 mb-4">"Countdown items in styled boxes."</p>
            {boxed_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Countdown, CountdownGroup, CountdownValue};",
                "use rsx_macros::rsx;",
                "",
                "// Basic standalone",
                "let html = rsx! {",
                "    <Countdown value={42} />",
                "};",
                "",
                "// Clock style (grouped)",
                "let html = rsx! {",
                "    <CountdownGroup class=\"font-mono text-2xl\">",
                "        <CountdownValue value={10} />",
                "        \"h \"",
                "        <CountdownValue value={24} />",
                "        \"m \"",
                "        <CountdownValue value={59} />",
                "        \"s\"",
                "    </CountdownGroup>",
                "};",
                "",
                "// With digits (minimum 2 digits)",
                "let html = rsx! {",
                "    <Countdown value={5} digits={2} />",
                "};",
                "",
                "// Dynamic (auto-decrement with JS)",
                "let html = rsx! {",
                "    <Countdown value={60} dynamic={true} />",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/countdown").await)
}
