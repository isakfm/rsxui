use axum::response::Html;
use rsx::rsx;
use rsxui::components::{Menu, MenuItem, MenuState};

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <Menu class="bg-base-200 w-56 rounded-box">
            <MenuItem>"Item 1"</MenuItem>
            <MenuItem>"Item 2"</MenuItem>
            <MenuItem>"Item 3"</MenuItem>
        </Menu>
    };

    let horizontal_example = rsx! {
        <Menu class="menu menu-horizontal bg-base-200 rounded-box">
            <MenuItem>"Item 1"</MenuItem>
            <MenuItem>"Item 2"</MenuItem>
            <MenuItem>"Item 3"</MenuItem>
        </Menu>
    };

    let active_example = rsx! {
        <Menu class="bg-base-200 w-56 rounded-box">
            <MenuItem>"Item 1"</MenuItem>
            <MenuItem state=MenuState::Active>"Item 2"</MenuItem>
            <MenuItem>"Item 3"</MenuItem>
        </Menu>
    };

    let disabled_example = rsx! {
        <Menu class="bg-base-200 w-56 rounded-box">
            <MenuItem>"Enabled item"</MenuItem>
            <MenuItem state=MenuState::Disabled>"Disabled item"</MenuItem>
            <MenuItem state=MenuState::Disabled>"Disabled item"</MenuItem>
        </Menu>
    };

    let focus_example = rsx! {
        <Menu class="bg-base-200 w-56 rounded-box">
            <MenuItem>"Item 1"</MenuItem>
            <MenuItem state=MenuState::Focus>"Item 2 (focus)"</MenuItem>
            <MenuItem>"Item 3"</MenuItem>
        </Menu>
    };

    let responsive_example = rsx! {
        <Menu class="menu menu-vertical lg:menu-horizontal bg-base-200 rounded-box">
            <MenuItem>"Item 1"</MenuItem>
            <MenuItem>"Item 2"</MenuItem>
            <MenuItem>"Item 3"</MenuItem>
        </Menu>
    };

    let with_icons_example = rsx! {
        <Menu class="bg-base-200 w-56 rounded-box">
            <MenuItem>
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
                </svg>
                " Home"
            </MenuItem>
            <MenuItem>
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
                " About"
            </MenuItem>
            <MenuItem>
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
                </svg>
                " Settings"
            </MenuItem>
        </Menu>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Menu"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Menu is used to display a list of links vertically or horizontally."
            </p>

            <div class="divider">"Basic Menu"</div>
            <p class="text-sm text-gray-600 mb-4">"A simple vertical menu with buttons."</p>
            {basic_example}

            <div class="divider">"Horizontal Menu"</div>
            <p class="text-sm text-gray-600 mb-4">"Menu displayed horizontally using `menu-horizontal` class."</p>
            {horizontal_example}

            <div class="divider">"Menu with Active Item"</div>
            <p class="text-sm text-gray-600 mb-4">"Using `MenuState::Active` to highlight the active menu item."</p>
            {active_example}

            <div class="divider">"Menu with Disabled Items"</div>
            <p class="text-sm text-gray-600 mb-4">"Using `MenuState::Disabled` for disabled menu items."</p>
            {disabled_example}

            <div class="divider">"Menu with Focus State"</div>
            <p class="text-sm text-gray-600 mb-4">"Using `MenuState::Focus` for focused menu items."</p>
            {focus_example}

            <div class="divider">"Responsive Menu"</div>
            <p class="text-sm text-gray-600 mb-4">"Vertical on small screens, horizontal on large screens."</p>
            {responsive_example}

            <div class="divider">"Menu with Icons"</div>
            <p class="text-sm text-gray-600 mb-4">"Menu items with SVG icons."</p>
            {with_icons_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Menu, MenuItem, MenuState};",
                "use rsx::rsx;",
                "",
                "let html = rsx! {",
                "    <Menu class=\"bg-base-200 w-56 rounded-box\">",
                "        <MenuItem>\"Item 1\"</MenuItem>",
                "        <MenuItem>\"Item 2\"</MenuItem>",
                "        <MenuItem>\"Item 3\"</MenuItem>",
                "    </Menu>",
                "};",
                "",
                "// Or with states:",
                "let html = rsx! {",
                "    <Menu class=\"bg-base-200 w-56 rounded-box\">",
                "        <MenuItem>\"Normal item\"</MenuItem>",
                "        <MenuItem state=MenuState::Active}>\"Active item\"</MenuItem>",
                "        <MenuItem state=MenuState::Disabled}>\"Disabled item\"</MenuItem>",
                "    </Menu>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/menu").await)
}
