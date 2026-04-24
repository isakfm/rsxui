use axum::response::Html;
use rsx_macros::rsx;
use rsxui::components::{Tab, TabContent, TabRadio, TabStyle, Tabs};

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <Tabs>
            <Tab active=true>"Tab 1"</Tab>
            <Tab>"Tab 2"</Tab>
            <Tab>"Tab 3"</Tab>
        </Tabs>
    };

    let styles_example = rsx! {
        <div class="flex flex-col gap-4">
            <Tabs style=TabStyle::Box>
                <Tab>"Box 1"</Tab>
                <Tab active=true>"Box 2"</Tab>
                <Tab>"Box 3"</Tab>
            </Tabs>
            <Tabs style=TabStyle::Border>
                <Tab>"Border 1"</Tab>
                <Tab active=true>"Border 2"</Tab>
                <Tab>"Border 3"</Tab>
            </Tabs>
            <Tabs style=TabStyle::Lift>
                <Tab>"Lift 1"</Tab>
                <Tab active=true>"Lift 2"</Tab>
                <Tab>"Lift 3"</Tab>
            </Tabs>
        </div>
    };

    let radio_example = rsx! {
        <Tabs style=TabStyle::Box>
            <TabRadio name="my_tabs_2" aria_label="Tab 1" checked=true />
            <TabRadio name="my_tabs_2" aria_label="Tab 2" />
            <TabRadio name="my_tabs_2" aria_label="Tab 3" />
        </Tabs>
    };

    let with_content_example = rsx! {
        <div>
            <Tabs style=TabStyle::Lift>
                <Tab active=true>"Details"</Tab>
                <Tab>"Reviews"</Tab>
                <Tab disabled=true>"Settings"</Tab>
            </Tabs>
            <TabContent class="bg-base-200 border-base-300 rounded-box p-6">
                "Tab content goes here. Use radio inputs for functional tab switching."
            </TabContent>
        </div>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Tab"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Tabs can be used to show a list of links in a tabbed format."
            </p>

            <div class="divider">"Basic Tabs"</div>
            {basic_example}

            <div class="divider">"Styles"</div>
            <p class="text-sm text-gray-600 mb-4">"Box, border, and lift styles."</p>
            {styles_example}

            <div class="divider">"Radio Tabs"</div>
            <p class="text-sm text-gray-600 mb-4">"Use radio inputs for tab switching."</p>
            {radio_example}

            <div class="divider">"With Content"</div>
            <p class="text-sm text-gray-600 mb-4">"Combine tabs with tab content panels."</p>
            {with_content_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Tabs, Tab, TabStyle};",
                "use rsx_macros::rsx;",
                "",
                "let html = rsx! {",
                "    <Tabs style=TabStyle::Lift>",
                "        <Tab active=true>\"Tab 1\"</Tab>",
                "        <Tab>\"Tab 2\"</Tab>",
                "        <Tab disabled=true>\"Tab 3\"</Tab>",
                "    </Tabs>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/tab").await)
}
