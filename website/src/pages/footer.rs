use axum::response::Html;
use rsx::rsx;
use rsxui::components::{Footer, FooterTitle};

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <Footer class="bg-base-200 p-10">
            <nav>
                <FooterTitle>"Services"</FooterTitle>
                <a class="link link-hover">"Branding"</a>
                <a class="link link-hover">"Design"</a>
                <a class="link link-hover">"Marketing"</a>
                <a class="link link-hover">"Advertisement"</a>
            </nav>
            <nav>
                <FooterTitle>"Company"</FooterTitle>
                <a class="link link-hover">"About us"</a>
                <a class="link link-hover">"Contact"</a>
                <a class="link link-hover">"Jobs"</a>
                <a class="link link-hover">"Press kit"</a>
            </nav>
            <nav>
                <FooterTitle>"Legal"</FooterTitle>
                <a class="link link-hover">"Terms of use"</a>
                <a class="link link-hover">"Privacy policy"</a>
                <a class="link link-hover">"Cookie policy"</a>
            </nav>
        </Footer>
    };

    let centered_example = rsx! {
        <Footer class="bg-base-200 p-10" center=true>
            <aside>
                <svg width="50" height="50" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" fill-rule="evenodd" clip-rule="evenodd" class="fill-current inline-block">
                    <path d="M22.672 15.226l-2.432.811.841 2.515c.33 1.019-.747 2.129-1.788 1.788-1.019-.33-2.129-.747-1.788-1.788l.841-2.515-2.432-.811c-1.019-.33-1.019-1.788 0-2.117l2.432-.811-.841-2.515c-.33-1.019.747-2.129 1.788-1.788 1.019.33 2.129.747 1.788 1.788l-.841 2.515 2.432.811c1.019.33 1.019 1.788 0 2.117z" />
                </svg>
                <p class="font-bold">
                    "ACME Industries Ltd."
                    <br />
                    "Providing reliable tech since 1992"
                </p>
                <p>"Copyright 2024 - All right reserved"</p>
            </aside>
        </Footer>
    };

    let vertical_example = rsx! {
        <Footer class="bg-neutral text-neutral-content p-10" vertical=true>
            <aside>
                <svg width="50" height="50" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" fill-rule="evenodd" clip-rule="evenodd" class="fill-current inline-block">
                    <path d="M22.672 15.226l-2.432.811.841 2.515c.33 1.019-.747 2.129-1.788 1.788-1.019-.33-2.129-.747-1.788-1.788l.841-2.515-2.432-.811c-1.019-.33-1.019-1.788 0-2.117l2.432-.811-.841-2.515c-.33-1.019.747-2.129 1.788-1.788 1.019.33 2.129.747 1.788 1.788l-.841 2.515 2.432.811c1.019.33 1.019 1.788 0 2.117z" />
                </svg>
                <p class="font-bold">
                    "ACME Industries Ltd."
                    <br />
                    "Providing reliable tech since 1992"
                </p>
            </aside>
            <nav>
                <FooterTitle>"Social"</FooterTitle>
                <div class="grid grid-flow-col gap-4">
                    <a><svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" class="fill-current"><path d="M24 4.557c-.883.392-1.832.656-2.828.775 1.017-.609 1.798-1.574 2.165-2.724-.951.564-2.005.974-3.127 1.195-.897-.957-2.178-1.555-3.594-1.555-3.179 0-5.515 2.966-4.797 6.045-4.091-.205-7.719-2.165-10.148-5.144-1.29 2.213-.669 5.108 1.523 6.574-.806-.026-1.566-.247-2.229-.616-.054 2.281 1.581 4.415 3.949 4.89-.693.188-1.452.232-2.224.084.626 1.956 2.444 3.379 4.6 3.419-2.07 1.623-4.678 2.348-7.29 2.04 2.179 1.397 4.768 2.212 7.548 2.212 9.142 0 14.307-7.721 13.995-14.646.962-.695 1.797-1.562 2.457-2.549z"/></svg></a>
                    <a><svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" class="fill-current"><path d="M19.615 3.184c-3.604-.246-11.631-.245-15.23 0-3.897.266-4.356 2.62-4.385 8.816.029 6.185.484 8.549 4.385 8.816 3.6.245 11.626.246 15.23 0 3.897-.266 4.356-2.62 4.385-8.816-.029-6.185-.484-8.549-4.385-8.816zm-10.615 12.816v-8l8 3.993-8 4.007z"/></svg></a>
                    <a><svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" class="fill-current"><path d="M9 8h-3v4h3v12h5v-12h3.642l.358-4h-4v-1.667c0-.955.192-1.333 1.115-1.333h2.885v-5h-3.808c-3.596 0-5.192 1.583-5.192 4.615v3.385z"/></svg></a>
                </div>
            </nav>
        </Footer>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Footer"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Footer can contain logo, copyright notice, and links to other pages."
            </p>

            <div class="divider">"Basic Footer"</div>
            {basic_example}

            <div class="divider">"Centered Footer"</div>
            <p class="text-sm text-gray-600 mb-4">"Logo and copyright centered."</p>
            {centered_example}

            <div class="divider">"Vertical Footer"</div>
            <p class="text-sm text-gray-600 mb-4">"Stacked layout with social links."</p>
            {vertical_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Footer, FooterTitle};",
                "use rsx::rsx;",
                "",
                "let html = rsx! {",
                "    <Footer class=\"bg-base-200 p-10\">",
                "        <nav>",
                "            <FooterTitle>\"Services\"</FooterTitle>",
                "            <a class=\"link link-hover\">\"Branding\"</a>",
                "        </nav>",
                "    </Footer>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/footer").await)
}
