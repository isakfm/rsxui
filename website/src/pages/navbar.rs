use axum::response::Html;
use rsx::rsx;
use rsxui::components::{
    Avatar, Dropdown, DropdownContent, DropdownPlacement, Navbar, NavbarCenter, NavbarEnd,
    NavbarStart,
};

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <Navbar class="bg-base-200 rounded-box">
            <NavbarStart>
                <a class="btn btn-ghost text-xl">"daisyUI"</a>
            </NavbarStart>
            <NavbarCenter class="hidden lg:flex">
                <ul class="menu menu-horizontal px-1">
                    <li><a>"Home"</a></li>
                    <li><a>"About"</a></li>
                    <li><a>"Contact"</a></li>
                </ul>
            </NavbarCenter>
            <NavbarEnd>
                <a class="btn">"Button"</a>
            </NavbarEnd>
        </Navbar>
    };

    let responsive_example = rsx! {
        <Navbar class="bg-base-200 rounded-box">
            <NavbarStart>
                <Dropdown>
                    <div tabindex="0" role="button" class="btn btn-ghost lg:hidden">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h8m-8 6h16" />
                        </svg>
                    </div>
                    <DropdownContent class="bg-base-200 rounded-box z-1 mt-3 w-52 p-2 shadow">
                        <ul class="menu menu-sm">
                            <li><a>"Item 1"</a></li>
                            <li><a>"Item 2"</a></li>
                            <li><a>"Item 3"</a></li>
                        </ul>
                    </DropdownContent>
                </Dropdown>
                <a class="btn btn-ghost text-xl">"daisyUI"</a>
            </NavbarStart>
            <NavbarCenter class="hidden lg:flex">
                <ul class="menu menu-horizontal px-1">
                    <li><a>"Item 1"</a></li>
                    <li><a>"Item 2"</a></li>
                    <li><a>"Item 3"</a></li>
                </ul>
            </NavbarCenter>
            <NavbarEnd>
                <a class="btn">"Button"</a>
            </NavbarEnd>
        </Navbar>
    };

    let dropdown_example = rsx! {
        <Navbar class="bg-base-200 rounded-box">
            <NavbarStart>
                <a class="btn btn-ghost text-xl">"daisyUI"</a>
            </NavbarStart>
             <NavbarCenter class="hidden lg:flex">
                <ul class="menu menu-horizontal px-1">
                    <li><a>"Home"</a></li>
                    <li>
                        <details>
                            <summary>"Parent"</summary>
                            <ul class="bg-base-200 rounded-t-none p-2 z-1">
                                <li><a>"Sub 1"</a></li>
                                <li><a>"Sub 2"</a></li>
                            </ul>
                        </details>
                    </li>
                    <li><a>"Contact"</a></li>
                </ul>
            </NavbarCenter>
            <NavbarEnd>
                <Dropdown placement=DropdownPlacement::End>
                    <div tabindex="0" role="button" class="btn btn-ghost btn-circle">
                        <Avatar src="https://img.daisyui.com/images/stock/photo-1534528741775-53994a69daeb.webp" alt="User avatar" inner_class="w-10 rounded-full" />
                    </div>
                    <DropdownContent class="bg-base-200 rounded-box z-1 mt-3 w-52 p-2 shadow">
                        <ul class="menu menu-sm">
                            <li>
                                <a class="justify-between">
                                    "Profile"
                                    <span class="badge">"New"</span>
                                </a>
                            </li>
                            <li><a>"Settings"</a></li>
                            <li><a>"Logout"</a></li>
                        </ul>
                    </DropdownContent>
                </Dropdown>
            </NavbarEnd>
        </Navbar>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Navbar"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Navbar is used to show a navigation bar on the top of the page."
            </p>

            <div class="divider">"Basic Navbar"</div>
            {basic_example}

            <div class="divider">"Responsive Navbar"</div>
            <p class="text-sm text-gray-600 mb-4">"Hide menu on small screens."</p>
            {responsive_example}

            <div class="divider">"Navbar with Dropdown"</div>
            <p class="text-sm text-gray-600 mb-4">"Dropdown menu in navbar using Dropdown component and Avatar."</p>
            {dropdown_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Navbar, NavbarStart, NavbarCenter, NavbarEnd};",
                "use rsx::rsx;",
                "",
                "let html = rsx! {",
                "    <Navbar class=\"bg-base-200\">",
                "        <NavbarStart>",
                "            <a class=\"btn btn-ghost text-xl\">\"Logo\"</a>",
                "        </NavbarStart>",
                "        <NavbarCenter>",
                "            <ul class=\"menu menu-horizontal\">",
                "                <li><a>\"Home\"</a></li>",
                "            </ul>",
                "        </NavbarCenter>",
                "        <NavbarEnd>",
                "            <a class=\"btn\">\"Button\"</a>",
                "        </NavbarEnd>",
                "    </Navbar>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/navbar").await)
}
