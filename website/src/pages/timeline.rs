use axum::response::Html;
use rsx_macros::rsx;
use rsxui::components::{
    Timeline, TimelineDirection, TimelineEnd, TimelineItem, TimelineMiddle, TimelineModifier,
    TimelineStart,
};

use crate::layout;

fn check_icon() -> String {
    r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="h-5 w-5"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z" clip-rule="evenodd" /></svg>"#.to_string()
}

pub async fn page() -> Html<String> {
    let icon = check_icon();

    let basic_example = rsx! {
        <Timeline>
            <TimelineItem>
                <TimelineStart>"1984"</TimelineStart>
                <TimelineMiddle>{icon.clone()}</TimelineMiddle>
                <TimelineEnd class="timeline-box">"First Macintosh computer"</TimelineEnd>
                <hr />
            </TimelineItem>
            <TimelineItem>
                <hr />
                <TimelineStart>"1998"</TimelineStart>
                <TimelineMiddle>{icon.clone()}</TimelineMiddle>
                <TimelineEnd class="timeline-box">"iMac"</TimelineEnd>
                <hr />
            </TimelineItem>
            <TimelineItem>
                <hr />
                <TimelineStart>"2001"</TimelineStart>
                <TimelineMiddle>{icon.clone()}</TimelineMiddle>
                <TimelineEnd class="timeline-box">"iPod"</TimelineEnd>
                <hr />
            </TimelineItem>
            <TimelineItem>
                <hr />
                <TimelineStart>"2007"</TimelineStart>
                <TimelineMiddle>{icon.clone()}</TimelineMiddle>
                <TimelineEnd class="timeline-box">"iPhone"</TimelineEnd>
                <hr />
            </TimelineItem>
            <TimelineItem>
                <hr />
                <TimelineStart>"2015"</TimelineStart>
                <TimelineMiddle>{icon.clone()}</TimelineMiddle>
                <TimelineEnd class="timeline-box">"Apple Watch"</TimelineEnd>
            </TimelineItem>
        </Timeline>
    };

    let vertical_example = rsx! {
        <Timeline direction=TimelineDirection::Vertical>
            <TimelineItem>
                <TimelineStart>"1984"</TimelineStart>
                <TimelineMiddle>{icon.clone()}</TimelineMiddle>
                <TimelineEnd class="timeline-box">"First Macintosh computer"</TimelineEnd>
                <hr />
            </TimelineItem>
            <TimelineItem>
                <hr />
                <TimelineStart>"1998"</TimelineStart>
                <TimelineMiddle>{icon.clone()}</TimelineMiddle>
                <TimelineEnd class="timeline-box">"iMac"</TimelineEnd>
                <hr />
            </TimelineItem>
            <TimelineItem>
                <hr />
                <TimelineStart>"2001"</TimelineStart>
                <TimelineMiddle>{icon.clone()}</TimelineMiddle>
                <TimelineEnd class="timeline-box">"iPod"</TimelineEnd>
                <hr />
            </TimelineItem>
        </Timeline>
    };

    let snap_icon_example = rsx! {
        <Timeline modifier=TimelineModifier::SnapIcon>
            <TimelineItem>
                <TimelineStart>"1984"</TimelineStart>
                <TimelineMiddle>{icon.clone()}</TimelineMiddle>
                <TimelineEnd class="timeline-box">"First Macintosh computer"</TimelineEnd>
                <hr />
            </TimelineItem>
            <TimelineItem>
                <hr />
                <TimelineStart>"1998"</TimelineStart>
                <TimelineMiddle>{icon.clone()}</TimelineMiddle>
                <TimelineEnd class="timeline-box">"iMac"</TimelineEnd>
                <hr />
            </TimelineItem>
            <TimelineItem>
                <hr />
                <TimelineStart>"2001"</TimelineStart>
                <TimelineMiddle>{icon.clone()}</TimelineMiddle>
                <TimelineEnd class="timeline-box">"iPod"</TimelineEnd>
                <hr />
            </TimelineItem>
        </Timeline>
    };

    let bottom_only_example = rsx! {
        <Timeline>
            <TimelineItem>
                <TimelineMiddle>{icon.clone()}</TimelineMiddle>
                <TimelineEnd class="timeline-box">"First Macintosh computer"</TimelineEnd>
                <hr />
            </TimelineItem>
            <TimelineItem>
                <hr />
                <TimelineMiddle>{icon.clone()}</TimelineMiddle>
                <TimelineEnd class="timeline-box">"iMac"</TimelineEnd>
                <hr />
            </TimelineItem>
            <TimelineItem>
                <hr />
                <TimelineMiddle>{icon.clone()}</TimelineMiddle>
                <TimelineEnd class="timeline-box">"iPod"</TimelineEnd>
                <hr />
            </TimelineItem>
        </Timeline>
    };

    let top_only_example = rsx! {
        <Timeline>
            <TimelineItem>
                <TimelineStart class="timeline-box">"First Macintosh computer"</TimelineStart>
                <TimelineMiddle>{icon.clone()}</TimelineMiddle>
                <hr />
            </TimelineItem>
            <TimelineItem>
                <hr />
                <TimelineStart class="timeline-box">"iMac"</TimelineStart>
                <TimelineMiddle>{icon.clone()}</TimelineMiddle>
                <hr />
            </TimelineItem>
            <TimelineItem>
                <hr />
                <TimelineStart class="timeline-box">"iPod"</TimelineStart>
                <TimelineMiddle>{icon.clone()}</TimelineMiddle>
                <hr />
            </TimelineItem>
        </Timeline>
    };

    let compact_example = rsx! {
        <Timeline modifier=TimelineModifier::Compact>
            <TimelineItem>
                <TimelineStart class="me-2">"1984"</TimelineStart>
                <TimelineMiddle>{icon.clone()}</TimelineMiddle>
                <TimelineEnd class="timeline-box">"First Macintosh computer"</TimelineEnd>
                <hr />
            </TimelineItem>
            <TimelineItem>
                <hr />
                <TimelineStart class="me-2">"1998"</TimelineStart>
                <TimelineMiddle>{icon.clone()}</TimelineMiddle>
                <TimelineEnd class="timeline-box">"iMac"</TimelineEnd>
                <hr />
            </TimelineItem>
            <TimelineItem>
                <hr />
                <TimelineStart class="me-2">"2001"</TimelineStart>
                <TimelineMiddle>{icon.clone()}</TimelineMiddle>
                <TimelineEnd class="timeline-box">"iPod"</TimelineEnd>
                <hr />
            </TimelineItem>
        </Timeline>
    };

    let horizontal_example = rsx! {
        <Timeline direction=TimelineDirection::Horizontal>
            <TimelineItem>
                <TimelineStart>"Start"</TimelineStart>
                <TimelineMiddle>{icon.clone()}</TimelineMiddle>
                <TimelineEnd class="timeline-box">"End"</TimelineEnd>
                <hr />
            </TimelineItem>
            <TimelineItem>
                <hr />
                <TimelineStart>"Start"</TimelineStart>
                <TimelineMiddle>{icon.clone()}</TimelineMiddle>
                <TimelineEnd class="timeline-box">"End"</TimelineEnd>
                <hr />
            </TimelineItem>
            <TimelineItem>
                <hr />
                <TimelineStart>"Start"</TimelineStart>
                <TimelineMiddle>{icon.clone()}</TimelineMiddle>
                <TimelineEnd class="timeline-box">"End"</TimelineEnd>
            </TimelineItem>
        </Timeline>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Timeline"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Timeline component shows a list of events in chronological order."
            </p>

            <div class="alert alert-info mb-8">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="h-6 w-6 shrink-0 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                <span>"Add <code>&lt;hr /&gt;</code> inside each item to display connector lines between events."</span>
            </div>

            <div class="divider">"Basic Timeline"</div>
            {basic_example}

            <div class="divider">"Vertical"</div>
            <p class="text-sm text-gray-600 mb-4">"Explicit vertical direction."</p>
            {vertical_example}

            <div class="divider">"Snap Icon"</div>
            <p class="text-sm text-gray-600 mb-4">"Snaps the icon to the start instead of middle."</p>
            {snap_icon_example}

            <div class="divider">"Bottom Side Only"</div>
            <p class="text-sm text-gray-600 mb-4">"Content only on the bottom/end side."</p>
            {bottom_only_example}

            <div class="divider">"Top Side Only"</div>
            <p class="text-sm text-gray-600 mb-4">"Content only on the top/start side."</p>
            {top_only_example}

            <div class="divider">"Compact"</div>
            <p class="text-sm text-gray-600 mb-4">"Forces all items on one side."</p>
            {compact_example}

            <div class="divider">"Horizontal"</div>
            <p class="text-sm text-gray-600 mb-4">"Horizontal layout."</p>
            {horizontal_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Timeline, TimelineItem, TimelineStart, TimelineMiddle, TimelineEnd};",
                "use rsx_macros::rsx;",
                "",
                "let html = rsx! {",
                "    <Timeline>",
                "        <TimelineItem>",
                "            <TimelineStart>\"1984\"</TimelineStart>",
                "            <TimelineMiddle>\"●\"</TimelineMiddle>",
                "            <TimelineEnd class=\"timeline-box\">\"First Mac\"</TimelineEnd>",
                "            <hr />",
                "        </TimelineItem>",
                "    </Timeline>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/timeline").await)
}
