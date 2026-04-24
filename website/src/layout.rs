use rsx_macros::rsx;
use rsxui::components::{
    Drawer, DrawerContent, DrawerOverlay, DrawerSide, DrawerToggle, Menu, MenuItem, MenuState,
};

pub async fn drawer(content: String, active_path: &str) -> String {
    let sidebar_items = [
        ("/", "Home"),
        ("/components/accordion", "Accordion"),
        ("/components/alert", "Alert"),
        ("/components/avatar", "Avatar"),
        ("/components/badge", "Badge"),
        ("/components/browser_mockup", "Browser Mockup"),
        ("/components/breadcrumb", "Breadcrumb"),
        ("/components/button", "Button"),
        ("/components/card", "Card"),
        ("/components/chat", "Chat"),
        ("/components/checkbox", "Checkbox"),
        ("/components/code_mockup", "Code Mockup"),
        ("/components/collapse", "Collapse"),
        ("/components/countdown", "Countdown"),
        ("/components/diff", "Diff"),
        ("/components/divider", "Divider"),
        ("/components/dock", "Dock"),
        ("/components/drawer", "Drawer"),
        ("/components/dropdown", "Dropdown"),
        ("/components/fieldset", "Fieldset"),
        ("/components/file_input", "File Input"),
        ("/components/filter", "Filter"),
        ("/components/footer", "Footer"),
        ("/components/hero", "Hero"),
        ("/components/indicator", "Indicator"),
        ("/components/input", "Input"),
        ("/components/join", "Join"),
        ("/components/kbd", "Kbd"),
        ("/components/label", "Label"),
        ("/components/link", "Link"),
        ("/components/list", "List"),
        ("/components/loading", "Loading"),
        ("/components/menu", "Menu"),
        ("/components/navbar", "Navbar"),
        ("/components/pagination", "Pagination"),
        ("/components/phone_mockup", "Phone Mockup"),
        ("/components/progress", "Progress"),
        ("/components/radio", "Radio"),
        ("/components/range", "Range"),
        ("/components/rating", "Rating"),
        ("/components/select", "Select"),
        ("/components/skeleton", "Skeleton"),
        ("/components/stat", "Stat"),
        ("/components/status", "Status"),
        ("/components/steps", "Steps"),
        ("/components/swap", "Swap"),
        ("/components/tab", "Tab"),
        ("/components/table", "Table"),
        ("/components/textarea", "Textarea"),
        ("/components/textrotate", "Text Rotate"),
        ("/components/toast", "Toast"),
        ("/components/toggle", "Toggle"),
        ("/components/tooltip", "Tooltip"),
        ("/components/validator", "Validator"),
        ("/components/window_mockup", "Window Mockup"),
    ];

    let mut menu_items = Vec::new();
    for (path, label) in sidebar_items.iter() {
        let state = if *path == active_path {
            MenuState::Active
        } else {
            MenuState::Normal
        };
        let item = rsx! {
            <MenuItem state=state url=path.to_string()>{label}</MenuItem>
        };
        menu_items.push(item);
    }
    let menu_html = menu_items.join("");

    rsx! {
        <!DOCTYPE html>
        <html data-theme="light">
        <head>
            <meta charset="UTF-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            <title>"RsxUI - Rust Jsx-like DaisyUI Components"</title>
            <link href="https://cdn.jsdelivr.net/npm/daisyui@5" rel="stylesheet" type="text/css" />
            <script src="https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4"></script>
            <link href="/static/css/style.css" rel="stylesheet" />
        </head>
        <body>
            <Drawer id="main-drawer" open=true>
                <DrawerToggle id="main-drawer" />
                <DrawerContent class="p-5">
                    <div class="navbar bg-base-100 lg:hidden">
                        <div class="flex-none">
                            <label for="main-drawer" class="btn btn-square btn-ghost drawer-button">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-6 h-6 stroke-current">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>
                                </svg>
                            </label>
                        </div>
                        <div class="flex-1">
                            <span class="text-xl font-bold">RsxUI</span>
                        </div>
                    </div>
                    {content}
                </DrawerContent>
                <DrawerSide class="z-40">
                    <DrawerOverlay for_id="main-drawer" />
                    <div class="bg-base-200 text-base-content min-h-full w-72 p-4 flex flex-col justify-between">
                        <div>
                            <div class="mb-4">
                                <span class="text-2xl font-bold">RsxUI</span>
                                <p class="text-sm text-gray-500">DaisyUI Components for Rust</p>
                            </div>
                            <Menu class="menu menu-sm">
                                {menu_html}
                            </Menu>
                        </div>
                        <div class="mt-4 pt-4 border-t border-base-300">
                            <a href="https://github.com/isakfm/rsxui" target="_blank" class="flex items-center gap-2 text-sm hover:text-primary transition-colors mb-3">
                                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
                                    <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                                </svg>
                                "GitHub"
                            </a>
                            <p class="text-xs text-gray-400">"Created by Isak Rickyanto@2026"</p>
                        </div>
                    </div>
                </DrawerSide>
            </Drawer>
        </body>
        </html>
    }
}
