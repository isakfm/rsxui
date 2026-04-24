use axum::response::Html;
use rsx_macros::rsx;
use rsxui::components::{AccordionDetails, AccordionItem, AccordionModifier};

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <div class="flex flex-col gap-2">
            <AccordionItem name="accordion-1" title="How do I create an account?" checked=true class="bg-base-100 border border-base-300">
                "Click the Sign Up button in the top right corner and follow the registration process."
            </AccordionItem>
            <AccordionItem name="accordion-1" title="I forgot my password. What should I do?" class="bg-base-100 border border-base-300">
                "Click on Forgot Password on the login page and follow the instructions sent to your email."
            </AccordionItem>
            <AccordionItem name="accordion-1" title="How do I update my profile information?" class="bg-base-100 border border-base-300">
                "Go to My Account settings and select Edit Profile to make changes."
            </AccordionItem>
        </div>
    };

    let details_example = rsx! {
        <div class="flex flex-col gap-2">
            <AccordionDetails name="accordion-det-1" title="How do I create an account?" open=true class="bg-base-100 border border-base-300">
                "Click the Sign Up button in the top right corner and follow the registration process."
            </AccordionDetails>
            <AccordionDetails name="accordion-det-1" title="I forgot my password. What should I do?" class="bg-base-100 border border-base-300">
                "Click on Forgot Password on the login page and follow the instructions sent to your email."
            </AccordionDetails>
            <AccordionDetails name="accordion-det-1" title="How do I update my profile information?" class="bg-base-100 border border-base-300">
                "Go to My Account settings and select Edit Profile to make changes."
            </AccordionDetails>
        </div>
    };

    let arrow_example = rsx! {
        <div class="flex flex-col gap-2">
            <AccordionItem name="accordion-2" title="How do I create an account?" checked=true modifier=AccordionModifier::Arrow class="bg-base-100 border border-base-300">
                "Click the Sign Up button in the top right corner and follow the registration process."
            </AccordionItem>
            <AccordionItem name="accordion-2" title="I forgot my password. What should I do?" modifier=AccordionModifier::Arrow class="bg-base-100 border border-base-300">
                "Click on Forgot Password on the login page and follow the instructions sent to your email."
            </AccordionItem>
            <AccordionItem name="accordion-2" title="How do I update my profile information?" modifier=AccordionModifier::Arrow class="bg-base-100 border border-base-300">
                "Go to My Account settings and select Edit Profile to make changes."
            </AccordionItem>
        </div>
    };

    let plus_example = rsx! {
        <div class="flex flex-col gap-2">
            <AccordionItem name="accordion-3" title="How do I create an account?" checked=true modifier=AccordionModifier::Plus class="bg-base-100 border border-base-300">
                "Click the Sign Up button in the top right corner and follow the registration process."
            </AccordionItem>
            <AccordionItem name="accordion-3" title="I forgot my password. What should I do?" modifier=AccordionModifier::Plus class="bg-base-100 border border-base-300">
                "Click on Forgot Password on the login page and follow the instructions sent to your email."
            </AccordionItem>
            <AccordionItem name="accordion-3" title="How do I update my profile information?" modifier=AccordionModifier::Plus class="bg-base-100 border border-base-300">
                "Go to My Account settings and select Edit Profile to make changes."
            </AccordionItem>
        </div>
    };

    let join_example = rsx! {
        <div class="join join-vertical bg-base-100">
            <AccordionItem name="accordion-4" title="How do I create an account?" checked=true modifier=AccordionModifier::Arrow class="join-item border border-base-300">
                "Click the Sign Up button in the top right corner and follow the registration process."
            </AccordionItem>
            <AccordionItem name="accordion-4" title="I forgot my password. What should I do?" modifier=AccordionModifier::Arrow class="join-item border border-base-300">
                "Click on Forgot Password on the login page and follow the instructions sent to your email."
            </AccordionItem>
            <AccordionItem name="accordion-4" title="How do I update my profile information?" modifier=AccordionModifier::Arrow class="join-item border border-base-300">
                "Go to My Account settings and select Edit Profile to make changes."
            </AccordionItem>
        </div>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Accordion"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Accordion is used for showing and hiding content but only one item can stay open at a time."
            </p>

            <div class="divider">"Radio Inputs"</div>
            <p class="text-sm text-gray-600 mb-4">"Classic accordion using radio inputs. Only one item can be open at a time."</p>
            {basic_example}

            <div class="divider">"Using Details Element"</div>
            <p class="text-sm text-gray-600 mb-4">"Accordion using native HTML details element. Content is searchable in browser."</p>
            {details_example}

            <div class="divider">"With Arrow Icon"</div>
            <p class="text-sm text-gray-600 mb-4">"Arrow indicator showing open/close state."</p>
            {arrow_example}

            <div class="divider">"With Plus/Minus Icon"</div>
            <p class="text-sm text-gray-600 mb-4">"Plus/minus indicator for expand/collapse."</p>
            {plus_example}

            <div class="divider">"With Join"</div>
            <p class="text-sm text-gray-600 mb-4">"Accordion items joined together with automatic border radius handling."</p>
            {join_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{AccordionItem, AccordionModifier};",
                "use rsx_macros::rsx;",
                "",
                "let html = rsx! {",
                "    <AccordionItem",
                "        name=\"my-accordion\"",
                "        title=\"How do I create an account?\"",
                "        checked=true",
                "        class=\"bg-base-100 border border-base-300\"",
                "    >",
                "        \"Click the Sign Up button...\"",
                "    </AccordionItem>",
                "};",
                "",
                "// With arrow",
                "let html = rsx! {",
                "    <AccordionItem",
                "        name=\"my-accordion\"",
                "        title=\"Question\"",
                "        modifier={AccordionModifier::Arrow}",
                "    >",
                "        \"Answer...\"",
                "    </AccordionItem>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/accordion").await)
}
