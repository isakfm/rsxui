use axum::response::Html;
use rsx::rsx;
use rsxui::components::{Dropdown, DropdownContent, DropdownModifier, DropdownPlacement};

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <Dropdown>
            <div tabindex="0" role="button" class="btn m-1">"Click"</div>
            <DropdownContent class="p-2 shadow menu bg-base-100 rounded-box w-52 z-[1]">
                <ul>
                    <li><a>"Item 1"</a></li>
                    <li><a>"Item 2"</a></li>
                </ul>
            </DropdownContent>
        </Dropdown>
    };

    let hover_example = rsx! {
        <Dropdown modifier=DropdownModifier::Hover>
            <div tabindex="0" role="button" class="btn m-1">"Hover"</div>
            <DropdownContent class="p-2 shadow menu bg-base-100 rounded-box w-52 z-[1]">
                <ul>
                    <li><a>"Item 1"</a></li>
                    <li><a>"Item 2"</a></li>
                </ul>
            </DropdownContent>
        </Dropdown>
    };

    let open_example = rsx! {
        <Dropdown modifier=DropdownModifier::Open>
            <div tabindex="0" role="button" class="btn m-1">"Force Open"</div>
            <DropdownContent class="p-2 shadow menu bg-base-100 rounded-box w-52 z-[1]">
                <ul>
                    <li><a>"Item 1"</a></li>
                    <li><a>"Item 2"</a></li>
                </ul>
            </DropdownContent>
        </Dropdown>
    };

    let positions_example = rsx! {
        <div class="flex flex-wrap gap-4">
            <Dropdown placement=DropdownPlacement::Top>
                <div tabindex="0" role="button" class="btn m-1">"Top"</div>
                <DropdownContent class="p-2 shadow menu bg-base-100 rounded-box w-52 z-[1]">
                    <ul><li><a>"Item"</a></li></ul>
                </DropdownContent>
            </Dropdown>
            <Dropdown placement=DropdownPlacement::End>
                <div tabindex="0" role="button" class="btn m-1">"End"</div>
                <DropdownContent class="p-2 shadow menu bg-base-100 rounded-box w-52 z-[1]">
                    <ul><li><a>"Item"</a></li></ul>
                </DropdownContent>
            </Dropdown>
        </div>
    };

    let card_example = rsx! {
        <Dropdown>
            <div tabindex="0" role="button" class="btn m-1">"Card"</div>
            <DropdownContent class="card card-sm w-64 shadow-md bg-base-100 z-[1]">
                <div class="card-body">
                    <p>"This is a card inside a dropdown."</p>
                </div>
            </DropdownContent>
        </Dropdown>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Dropdown"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Dropdown can open a menu or any other element when the button is clicked."
            </p>

            <div class="divider">"Basic Dropdown"</div>
            {basic_example}

            <div class="divider">"Hover Dropdown"</div>
            <p class="text-sm text-gray-600 mb-4">"Opens on hover too."</p>
            {hover_example}

            <div class="divider">"Positions"</div>
            <p class="text-sm text-gray-600 mb-4">"Top and End placements."</p>
            {positions_example}

            <div class="divider">"Card Dropdown"</div>
            <p class="text-sm text-gray-600 mb-4">"Any element can be dropdown content."</p>
            {card_example}

            <div class="divider">"Force Open"</div>
            <p class="text-sm text-gray-600 mb-4">"Always visible with Open modifier."</p>
            {open_example}

            <div class="divider pt-20">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Dropdown, DropdownContent, DropdownModifier, DropdownPlacement};",
                "use rsx::rsx;",
                "",
                "let html = rsx! {",
                "    <Dropdown placement={DropdownPlacement::End} modifier={DropdownModifier::Hover}>",
                "        <div tabindex=\"0\" role=\"button\" class=\"btn\">\"Hover\"</div>",
                "        <DropdownContent>",
                "            <ul class=\"menu\"><li><a>\"Item\"</a></li></ul>",
                "        </DropdownContent>",
                "    </Dropdown>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/dropdown").await)
}
