use axum::response::Html;
use rsx::rsx;
use rsxui::components::{
    Drawer, DrawerButton, DrawerContent, DrawerOverlay, DrawerSide, DrawerToggle, Menu, MenuItem,
};

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <Drawer>
            <DrawerToggle id="drawer-basic" />
            <DrawerContent>
                <DrawerButton for_id="drawer-basic" label="Open Drawer" />
                <p class="p-4">"Main content area. Click the button to open the drawer."</p>
            </DrawerContent>
            <DrawerSide class="z-1020">
                <DrawerOverlay for_id="drawer-basic" />
                <Menu class="p-4 w-80 min-h-full bg-base-100 text-base-content">
                    <MenuItem>"Sidebar Item 1"</MenuItem>
                    <MenuItem>"Sidebar Item 2"</MenuItem>
                </Menu>
            </DrawerSide>
        </Drawer>
    };

    let navbar_example = rsx! {
        <Drawer>
            <DrawerToggle id="drawer-navbar" />
            <DrawerContent>
                <div class="w-full navbar bg-base-300">
                    <div class="flex-none lg:hidden">
                        <label for="drawer-navbar" aria-label="open sidebar" class="btn btn-square btn-ghost">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-6 h-6 stroke-current">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>
                            </svg>
                        </label>
                    </div>
                    <div class="flex-1 px-2 mx-2">Navbar Title</div>
                    <div class="flex-none hidden lg:block">
                        <Menu class="menu menu-horizontal">
                            <MenuItem>"Navbar Item 1"</MenuItem>
                            <MenuItem>"Navbar Item 2"</MenuItem>
                        </Menu>
                    </div>
                </div>
                <div class="flex justify-center items-center grow p-4">"Content"</div>
            </DrawerContent>
            <DrawerSide>
                <DrawerOverlay for_id="drawer-navbar" />
                <Menu class="p-4 w-80 min-h-full bg-base-200">
                    <MenuItem>"Sidebar Item 1"</MenuItem>
                    <MenuItem>"Sidebar Item 2"</MenuItem>
                </Menu>
            </DrawerSide>
        </Drawer>
    };

    let responsive_example = rsx! {
        <Drawer open=true class="h-200">
            <DrawerToggle id="drawer-responsive" />
            <DrawerContent>
                <DrawerButton for_id="drawer-responsive" show_on="lg:hidden" />
                <p class="p-4">"This drawer is always open on large screens."</p>
            </DrawerContent>
            <DrawerSide>
                <DrawerOverlay for_id="drawer-responsive" />
                <Menu class="p-4 w-80 min-h-200 bg-base-200">
                    <MenuItem>"Sidebar Item 1"</MenuItem>
                    <MenuItem>"Sidebar Item 2"</MenuItem>
                </Menu>
            </DrawerSide>
        </Drawer>
    };

    let right_example = rsx! {
        <Drawer placement=rsxui::components::DrawerPlacement::Right>
            <DrawerToggle id="drawer-right" />
            <DrawerContent>
                <DrawerButton for_id="drawer-right" label="Open Right" />
                <p class="p-4">"Main content area. Click the button to open the drawer from right."</p>
            </DrawerContent>
            <DrawerSide class="z-2002">
                <DrawerOverlay for_id="drawer-right" />
                <Menu class="p-4 w-80 min-h-full bg-base-200">
                    <MenuItem>"Sidebar Item 1"</MenuItem>
                    <MenuItem>"Sidebar Item 2"</MenuItem>
                </Menu>
            </DrawerSide>
        </Drawer>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Drawer"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Drawers provide responsive side navigation panels that can slide in from the edge of the viewport."
            </p>

            <div class="divider">"Basic Drawer"</div>
            <p class="text-sm text-gray-600 mb-4">"Click the button to open the drawer sidebar."</p>
            {basic_example}

            <div class="divider">"Navbar + Drawer"</div>
            <p class="text-sm text-gray-600 mb-4">"Desktop shows navbar menu, mobile shows drawer sidebar."</p>
            {navbar_example}

            <div class="divider">"Responsive Always Open"</div>
            <p class="text-sm text-gray-600 mb-4">"Sidebar always visible on large screens, toggleable on small screens."</p>
            {responsive_example}

            <div class="divider">"Drawer End (Right)"</div>
            <p class="text-sm text-gray-600 mb-4">"Drawer opens from the right side."</p>
            {right_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{",
                "    Drawer, DrawerToggle, DrawerContent, DrawerSide,",
                "    DrawerOverlay, DrawerButton, Menu, MenuItem,",
                "};",
                "use rsx::rsx;",
                "",
                "let html = rsx! {",
                "    <Drawer id=\"my-drawer\">",
                "        <DrawerToggle id=\"my-drawer\" />",
                "        <DrawerContent>",
                "            <DrawerButton for_id=\"my-drawer\" />",
                "            <p>\"Content\"</p>",
                "        </DrawerContent>",
                "        <DrawerSide>",
                "            <DrawerOverlay for_id=\"my-drawer\" />",
                "            <Menu class=\"p-4 w-80 min-h-full bg-base-200\">",
                "                <MenuItem>\"Item 1\"</MenuItem>",
                "                <MenuItem>\"Item 2\"</MenuItem>",
                "            </Menu>",
                "        </DrawerSide>",
                "    </Drawer>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/drawer").await)
}
