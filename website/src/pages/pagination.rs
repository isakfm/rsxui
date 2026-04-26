use axum::response::Html;
use rsx::rsx;
use rsxui::components::{Pagination, PaginationItem};

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <Pagination>
            <PaginationItem>"1"</PaginationItem>
            <PaginationItem active=true>"2"</PaginationItem>
            <PaginationItem>"3"</PaginationItem>
            <PaginationItem>"4"</PaginationItem>
        </Pagination>
    };

    let with_prev_next_example = rsx! {
        <Pagination>
            <PaginationItem>"«"</PaginationItem>
            <PaginationItem>"1"</PaginationItem>
            <PaginationItem active=true>"2"</PaginationItem>
            <PaginationItem>"3"</PaginationItem>
            <PaginationItem disabled=true>"..."</PaginationItem>
            <PaginationItem>"99"</PaginationItem>
            <PaginationItem>"»"</PaginationItem>
        </Pagination>
    };

    let sizes_example = rsx! {
        <Pagination>
            <PaginationItem class="btn-sm">"1"</PaginationItem>
            <PaginationItem class="btn-sm btn-active">"2"</PaginationItem>
            <PaginationItem class="btn-sm">"3"</PaginationItem>
        </Pagination>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Pagination"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Pagination is a group of buttons for page navigation."
            </p>

            <div class="divider">"Basic Pagination"</div>
            {basic_example}

            <div class="divider">"With Previous / Next"</div>
            <p class="text-sm text-gray-600 mb-4">"Add navigation arrows and ellipsis."</p>
            {with_prev_next_example}

            <div class="divider">"Small Size"</div>
            <p class="text-sm text-gray-600 mb-4">"Use Tailwind button size classes."</p>
            {sizes_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Pagination, PaginationItem};",
                "use rsx::rsx;",
                "",
                "let html = rsx! {",
                "    <Pagination>",
                "        <PaginationItem>\"1\"</PaginationItem>",
                "        <PaginationItem active=true>\"2\"</PaginationItem>",
                "        <PaginationItem>\"3\"</PaginationItem>",
                "    </Pagination>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/pagination").await)
}
