use axum::response::Html;
use rsx::rsx;
use rsxui::components::{Size, Table};

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <Table>
            <thead>
                <tr>
                    <th></th>
                    <th>"Name"</th>
                    <th>"Job"</th>
                    <th>"Favorite Color"</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <th>"1"</th>
                    <td>"Cy Ganderton"</td>
                    <td>"Quality Control Specialist"</td>
                    <td>"Blue"</td>
                </tr>
                <tr>
                    <th>"2"</th>
                    <td>"Hart Hagerty"</td>
                    <td>"Desktop Support Technician"</td>
                    <td>"Purple"</td>
                </tr>
                <tr>
                    <th>"3"</th>
                    <td>"Brice Swyre"</td>
                    <td>"Tax Accountant"</td>
                    <td>"Red"</td>
                </tr>
            </tbody>
        </Table>
    };

    let zebra_example = rsx! {
        <Table zebra=true>
            <thead>
                <tr>
                    <th></th>
                    <th>"Name"</th>
                    <th>"Job"</th>
                    <th>"Color"</th>
                </tr>
            </thead>
            <tbody>
                <tr><th>"1"</th><td>"Alice"</td><td>"Engineer"</td><td>"Blue"</td></tr>
                <tr><th>"2"</th><td>"Bob"</td><td>"Designer"</td><td>"Green"</td></tr>
                <tr><th>"3"</th><td>"Charlie"</td><td>"Manager"</td><td>"Red"</td></tr>
            </tbody>
        </Table>
    };

    let size_example = rsx! {
        <Table size=Size::Xs>
            <thead>
                <tr><th></th><th>"Name"</th><th>"Job"</th></tr>
            </thead>
            <tbody>
                <tr><th>"1"</th><td>"Alice"</td><td>"Engineer"</td></tr>
                <tr><th>"2"</th><td>"Bob"</td><td>"Designer"</td></tr>
            </tbody>
        </Table>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Table"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Table can be used to show a list of data in a table format."
            </p>

            <div class="divider">"Basic Table"</div>
            {basic_example}

            <div class="divider">"Zebra Table"</div>
            <p class="text-sm text-gray-600 mb-4">"Alternating row colors."</p>
            {zebra_example}

            <div class="divider">"Extra Small Table"</div>
            <p class="text-sm text-gray-600 mb-4">"Compact size for dense data."</p>
            {size_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Table, Size};",
                "use rsx::rsx;",
                "",
                "let html = rsx! {",
                "    <Table zebra={true} size={Size::Sm}>",
                "        <thead>",
                "            <tr><th>\"Name\"</th></tr>",
                "        </thead>",
                "        <tbody>",
                "            <tr><td>\"Alice\"</td></tr>",
                "        </tbody>",
                "    </Table>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/table").await)
}
