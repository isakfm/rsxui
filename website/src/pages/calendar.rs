use axum::response::Html;
use rsx::rsx;
use rsxui::components::CallyCalendar;

use crate::layout;

pub async fn page() -> Html<String> {
    let prev_icon = r#"<svg aria-label="Previous" class="fill-current size-4" slot="previous" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path fill="currentColor" d="M15.75 19.5 8.25 12l7.5-7.5"></path></svg>"#;
    let next_icon = r#"<svg aria-label="Next" class="fill-current size-4" slot="next" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path fill="currentColor" d="m8.25 4.5 7.5 7.5-7.5 7.5"></path></svg>"#;

    let cally_standalone = rsx! {
        <CallyCalendar class="bg-base-100 border border-base-300 shadow-lg rounded-box">
            {prev_icon}
            {next_icon}
            <calendar-month></calendar-month>
        </CallyCalendar>
    };

    let cally_btn_style = "anchor-name:--cally1";
    let cally_popover_style = "position-anchor:--cally1";
    let cally_onchange = "document.getElementById('cally1').innerText = this.value";

    let cally_datepicker = rsx! {
        <button popovertarget="cally-popover1" class="input input-border" id="cally1" style={cally_btn_style}>
            "Pick a date"
        </button>
        <div popover id="cally-popover1" class="dropdown bg-base-100 rounded-box shadow-lg" style={cally_popover_style}>
            <calendar-date class="cally" onchange={cally_onchange}>
                {prev_icon}
                {next_icon}
                <calendar-month></calendar-month>
            </calendar-date>
        </div>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Calendar"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Calendar component wrapping the Cally web component calendar."
            </p>

            <div class="divider">"Standalone Calendar"</div>
            <p class="text-sm text-gray-600 mb-4">"A simple inline calendar with DaisyUI styling."</p>
            <div class="mb-4">{cally_standalone}</div>

            <div class="divider">"Date Picker"</div>
            <p class="text-sm text-gray-600 mb-4">"Click the input to open a popover calendar. Uses CSS anchor positioning."</p>
            <div class="mb-4">{cally_datepicker}</div>

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::CallyCalendar;",
                "use rsx::rsx;",
                "",
                "let html = rsx! {",
                "    <CallyCalendar class=\"bg-base-100 border border-base-300 shadow-lg rounded-box\">",
                "        <svg aria-label=\"Previous\" slot=\"previous\" ...></svg>",
                "        <svg aria-label=\"Next\" slot=\"next\" ...></svg>",
                "        <calendar-month></calendar-month>",
                "    </CallyCalendar>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/calendar").await)
}
