use rsx_macros::rsx;
use rsxui::{
    components::{
        Drawer, DrawerContent, DrawerOverlay, DrawerSide, DrawerToggle, Menu, MenuItem, MenuState,
    },
    Size,
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
        ("/components/calendar", "Calendar"),
        ("/components/card", "Card"),
        ("/components/carousel", "Carousel"),
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
        ("/components/fab", "FAB"),
        ("/components/fieldset", "Fieldset"),
        ("/components/file_input", "File Input"),
        ("/components/filter", "Filter"),
        ("/components/footer", "Footer"),
        ("/components/hero", "Hero"),
        ("/components/hover3d", "Hover 3D"),
        ("/components/hover_gallery", "Hover Gallery"),
        ("/components/indicator", "Indicator"),
        ("/components/input", "Input"),
        ("/components/join", "Join"),
        ("/components/kbd", "Kbd"),
        ("/components/label", "Label"),
        ("/components/link", "Link"),
        ("/components/list", "List"),
        ("/components/loading", "Loading"),
        ("/components/mask", "Mask"),
        ("/components/menu", "Menu"),
        ("/components/modal", "Modal"),
        ("/components/navbar", "Navbar"),
        ("/components/pagination", "Pagination"),
        ("/components/phone_mockup", "Phone Mockup"),
        ("/components/progress", "Progress"),
        ("/components/radial_progress", "Radial Progress"),
        ("/components/radio", "Radio"),
        ("/components/range", "Range"),
        ("/components/rating", "Rating"),
        ("/components/select", "Select"),
        ("/components/skeleton", "Skeleton"),
        ("/components/stack", "Stack"),
        ("/components/stat", "Stat"),
        ("/components/status", "Status"),
        ("/components/steps", "Steps"),
        ("/components/swap", "Swap"),
        ("/components/tab", "Tab"),
        ("/components/table", "Table"),
        ("/components/textarea", "Textarea"),
        ("/components/textrotate", "Text Rotate"),
        ("/components/theme_controller", "Theme Controller"),
        ("/components/timeline", "Timeline"),
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

    let themes = [
        "light",
        "dark",
        "cupcake",
        "bumblebee",
        "emerald",
        "corporate",
        "synthwave",
        "retro",
        "cyberpunk",
        "valentine",
        "halloween",
        "garden",
        "forest",
        "aqua",
        "lofi",
        "pastel",
        "fantasy",
        "wireframe",
        "black",
        "luxury",
        "dracula",
        "cmyk",
        "autumn",
        "business",
        "acid",
        "lemonade",
        "night",
        "coffee",
        "winter",
        "dim",
        "nord",
        "sunset",
        "caramellatte",
        "abyss",
        "silk",
    ];

    let theme_items = themes
        .iter()
        .map(|t| {
            format!(
                r#"<li><a class="theme-option" data-theme="{}" onclick="setTheme('{}')">{}</a></li>"#,
                t, t, t
            )
        })
        .collect::<Vec<_>>()
        .join("");

    let theme_script = r#"
        (function() {
            const saved = localStorage.getItem('theme') || 'lofi';
            document.documentElement.setAttribute('data-theme', saved);
            updateThemeUI(saved);
        })();
        function setTheme(name) {
            document.documentElement.setAttribute('data-theme', name);
            localStorage.setItem('theme', name);
            updateThemeUI(name);
            const d = document.querySelector('details.dropdown');
            if (d) d.removeAttribute('open');
        }
        function updateThemeUI(name) {
            const label = document.getElementById('theme-label');
            if (label) label.textContent = name;
        }
        window.addEventListener('load', (event) => {
            const saved = localStorage.getItem('theme') || 'light';
            document.documentElement.setAttribute('data-theme', saved);
            updateThemeUI(saved);
        });
    "#;

    let css_inline = r#"
        .drawer-content {
            padding: 2rem;
        }
        .menu-title {
            font-weight: 600;
            text-transform: uppercase;
            font-size: 0.75rem;
            color: #666;
            margin-top: 1rem;
        }
        .code-wrapper {
            background-color: var(--color-neutral);
            border-radius: var(--radius-box);
        }
    "#;

    rsx! {
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset="UTF-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            <title>"RsxUI - Rust Jsx-like DaisyUI Components"</title>
            <link href="https://cdn.jsdelivr.net/npm/daisyui@5" rel="stylesheet" type="text/css" />
            <script src="https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4"></script>
            <link href="https://cdn.jsdelivr.net/npm/daisyui@5/themes.css" rel="stylesheet" type="text/css" />
            <script type="module" src="https://unpkg.com/cally"></script>
            <style>{css_inline}</style>
            <script>{theme_script}</script>
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
                            <div class="mb-4">
                                <label class="label">
                                    <span class="label-text text-xs font-semibold uppercase tracking-wider">"Theme"</span>
                                </label>
                                <details class="dropdown w-full">
                                    <summary class="btn btn-sm btn-outline w-full justify-between list-none cursor-pointer">
                                        <span id="theme-label">"light"</span>
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" /></svg>
                                    </summary>
                                    <ul class="menu menu-sm bg-base-100 rounded-box z-50 w-full max-h-48 overflow-y-auto shadow p-2 mt-1">
                                        {theme_items}
                                    </ul>
                                </details>
                            </div>
                            <div class="content-center">
                                <a href="https://github.com/isakfm/rsxui" target="_blank" class="flex items-center gap-2 text-sm hover:text-primary transition-colors mb-4 pl-2">
                                    <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
                                        <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                                    </svg>
                                    "Star our GitHub"
                                </a>
                            </div>
                            <Menu size=Size::Md class="w-full">
                                {menu_html}
                            </Menu>
                        </div>
                        <div class="mt-4 pt-4 border-t border-base-300">
                            <p class="text-xs text-gray-400">"Created by Isak Rickyanto@2026"</p>
                        </div>
                    </div>
                </DrawerSide>
            </Drawer>
        </body>
        </html>
    }
}
