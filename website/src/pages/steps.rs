use axum::response::Html;
use rsx::rsx;
use rsxui::components::{Color, Step, StepDirection, Steps};

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <Steps>
            <Step color=Color::Primary>"Register"</Step>
            <Step color=Color::Primary>"Choose plan"</Step>
            <Step>"Purchase"</Step>
        </Steps>
    };

    let data_content_example = rsx! {
        <Steps>
            <Step data_content="✓" color=Color::Primary>"Step 1"</Step>
            <Step data_content="✓" color=Color::Primary>"Step 2"</Step>
            <Step data_content="●">"Step 3"</Step>
        </Steps>
    };

    let vertical_example = rsx! {
        <Steps direction=StepDirection::Vertical>
            <Step color=Color::Secondary>"Receive product"</Step>
            <Step color=Color::Secondary>"Verify quality"</Step>
            <Step>"Ship to customer"</Step>
        </Steps>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Steps"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Steps can be used to show a list of steps in a process."
            </p>

            <div class="divider">"Basic Steps"</div>
            {basic_example}

            <div class="divider">"With Data Content"</div>
            <p class="text-sm text-gray-600 mb-4">"Use data-content for custom indicators."</p>
            {data_content_example}

            <div class="divider">"Vertical Steps"</div>
            <p class="text-sm text-gray-600 mb-4">"Stack steps vertically."</p>
            {vertical_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Steps, Step, Color, StepDirection};",
                "use rsx::rsx;",
                "",
                "let html = rsx! {",
                "    <Steps>",
                "        <Step color=Color::Primary>\"Register\"</Step>",
                "        <Step color=Color::Primary>\"Choose plan\"</Step>",
                "        <Step>\"Purchase\"</Step>",
                "    </Steps>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/steps").await)
}
