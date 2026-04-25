use axum::response::Html;
use rsx_macros::rsx;
use rsxui::components::{
    ThemeControllerButton, ThemeControllerCheckbox, ThemeControllerRadio, ThemeControllerToggle,
};

use crate::layout;

pub async fn page() -> Html<String> {
    let toggle_example = rsx! {
        <div class="flex items-center gap-4">
            <ThemeControllerToggle value="synthwave" />
            <span>"Toggle Theme"</span>
        </div>
    };

    let checkbox_example = rsx! {
        <div class="flex items-center gap-4">
            <ThemeControllerCheckbox value="dark" />
            <span>"Checkbox Theme"</span>
        </div>
    };

    let radio_example = rsx! {
        <fieldset class="fieldset">
            <label class="flex gap-2 cursor-pointer items-center">
                <ThemeControllerRadio name="theme-radios" value="default" />
                "Default"
            </label>
            <label class="flex gap-2 cursor-pointer items-center">
                <ThemeControllerRadio name="theme-radios" value="retro" />
                "Retro"
            </label>
            <label class="flex gap-2 cursor-pointer items-center">
                <ThemeControllerRadio name="theme-radios" value="cyberpunk" />
                "Cyberpunk"
            </label>
            <label class="flex gap-2 cursor-pointer items-center">
                <ThemeControllerRadio name="theme-radios" value="valentine" />
                "Valentine"
            </label>
            <label class="flex gap-2 cursor-pointer items-center">
                <ThemeControllerRadio name="theme-radios" value="aqua" />
                "Aqua"
            </label>
        </fieldset>
    };

    let button_example = rsx! {
        <div class="join join-vertical">
            <ThemeControllerButton name="theme-buttons" value="default" aria_label="Default" class="join-item" />
            <ThemeControllerButton name="theme-buttons" value="retro" aria_label="Retro" class="join-item" />
            <ThemeControllerButton name="theme-buttons" value="cyberpunk" aria_label="Cyberpunk" class="join-item" />
            <ThemeControllerButton name="theme-buttons" value="valentine" aria_label="Valentine" class="join-item" />
            <ThemeControllerButton name="theme-buttons" value="aqua" aria_label="Aqua" class="join-item" />
        </div>
    };

    let swap_example = rsx! {
        <label class="swap swap-rotate">
            <input type="checkbox" value="synthwave" class="theme-controller" />
            <svg class="swap-off fill-current w-10 h-10" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                <path d="M5.64,17l-.71.71a1,1,0,0,0,0,1.41,1,1,0,0,0,1.41,0l.71-.71A1,1,0,0,0,5.64,17ZM5,12a1,1,0,0,0-1-1H3a1,1,0,0,0,0,2H4A1,1,0,0,0,5,12Zm7-7a1,1,0,0,0,1-1V3a1,1,0,0,0-2,0V4A1,1,0,0,0,12,5ZM5.64,7.05a1,1,0,0,0,.7.29,1,1,0,0,0,.71-.29,1,1,0,0,0,0-1.41l-.71-.71A1,1,0,0,0,4.93,6.34Zm12,.29a1,1,0,0,0,.7-.29l.71-.71a1,1,0,1,0-1.41-1.41L17,5.64a1,1,0,0,0,0,1.41A1,1,0,0,0,17.66,7.34ZM21,11H20a1,1,0,0,0,0,2h1a1,1,0,0,0,0-2Zm-9,8a1,1,0,0,0-1,1v1a1,1,0,0,0,2,0V20A1,1,0,0,0,12,19ZM18.36,17A1,1,0,0,0,17,18.36l.71.71a1,1,0,0,0,1.41,0,1,1,0,0,0,0-1.41ZM12,6.5A5.5,5.5,0,1,0,17.5,12,5.51,5.51,0,0,0,12,6.5Zm0,9A3.5,3.5,0,1,1,15.5,12,3.5,3.5,0,0,1,12,15.5Z"/>
            </svg>
            <svg class="swap-on fill-current w-10 h-10" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                <path d="M21.64,13a1,1,0,0,0-1.05-.14,8.05,8.05,0,0,1-3.37.73A8.15,8.15,0,0,1,9.08,5.49a8.59,8.59,0,0,1,.25-2A1,1,0,0,0,8,2.36,10.14,10.14,0,1,0,22,14.05,1,1,0,0,0,21.64,13Zm-9.5,6.69A8.14,8.14,0,0,1,7.08,5.22v.27A10.15,10.15,0,0,0,17.22,15.63a9.79,9.79,0,0,0,2.1-.22A8.11,8.11,0,0,1,12.14,19.73Z"/>
            </svg>
        </label>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Theme Controller"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Theme Controller changes the page theme using CSS only. When a checked input with theme-controller class exists, the page uses that input's value as the theme."
            </p>

            <div class="divider">"Toggle (only works on Default)"</div>
            {toggle_example}

            <div class="divider">"Checkbox (only works on Default)"</div>
            {checkbox_example}

            <div class="divider">"Radio Inputs"</div>
            {radio_example}

            <div class="divider">"Button Radios"</div>
            {button_example}

            <div class="divider">"Swap with Icons"</div>
            {swap_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{ThemeControllerToggle, ThemeControllerRadio};",
                "use rsx_macros::rsx;",
                "",
                "let html = rsx! {",
                "    <ThemeControllerToggle value=\"synthwave\" />",
                "    <ThemeControllerRadio name=\"theme-radios\" value=\"retro\" />",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/theme_controller").await)
}
