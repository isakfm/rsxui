use axum::response::Html;
use rsx_macros::rsx;
use rsxui::components::{Navbar, NavbarCenter, NavbarEnd, NavbarStart};

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
                <div class="dropdown">
                    <div tabindex="0" role="button" class="btn btn-ghost lg:hidden">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h8m-8 6h16" />
                        </svg>
                    </div>
                </div>
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

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Navbar, NavbarStart, NavbarCenter, NavbarEnd};",
                "use rsx_macros::rsx;",
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
