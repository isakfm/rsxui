use axum::response::Html;
use rsx::rsx;
use rsxui::components::{List, ListRow};

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <List class="bg-base-100 rounded-box shadow-md">
            <ListRow>
                <div>
                    <div class="font-bold">"DaisyUI"</div>
                    <div class="text-sm opacity-50">"A popular component library"</div>
                </div>
            </ListRow>
            <ListRow>
                <div>
                    <div class="font-bold">"Tailwind CSS"</div>
                    <div class="text-sm opacity-50">"A utility-first CSS framework"</div>
                </div>
            </ListRow>
            <ListRow>
                <div>
                    <div class="font-bold">"Rust"</div>
                    <div class="text-sm opacity-50">"A systems programming language"</div>
                </div>
            </ListRow>
        </List>
    };

    let grow_example = rsx! {
        <List class="bg-base-100 rounded-box shadow-md">
            <ListRow>
                <div class="text-lg font-bold">"Title"</div>
                <div class="text-sm opacity-50">"Description that grows"</div>
            </ListRow>
            <ListRow>
                <div class="text-lg font-bold">"Another"</div>
                <div class="text-sm opacity-50">"More description text"</div>
            </ListRow>
        </List>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"List"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "List is a vertical layout to display information in rows."
            </p>

            <div class="divider">"Basic List"</div>
            {basic_example}

            <div class="divider">"List with Columns"</div>
            <p class="text-sm text-gray-600 mb-4">"Rows with multiple columns."</p>
            {grow_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{List, ListRow};",
                "use rsx::rsx;",
                "",
                "let html = rsx! {",
                "    <List>",
                "        <ListRow>\"Item 1\"</ListRow>",
                "        <ListRow>\"Item 2\"</ListRow>",
                "    </List>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/list").await)
}
