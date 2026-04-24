use axum::response::Html;
use rsx_macros::rsx;
use rsxui::components::{Button, ButtonModifier, ButtonStyle, ButtonType, Color, Size};

use crate::layout;

pub async fn page() -> Html<String> {
    // Basic examples
    let basic_examples = rsx! {
        <div class="flex flex-wrap gap-4">
            <Button label="Default" />
            <Button label="Primary" color=Color::Primary />
            <Button label="Secondary" color=Color::Secondary />
            <Button label="Accent" color=Color::Accent />
        </div>
    };

    // Size examples
    let size_examples = rsx! {
        <div class="flex flex-wrap items-center gap-4">
            <Button label="Extra Small" size=Size::Xs />
            <Button label="Small" size=Size::Sm />
            <Button label="Medium" size=Size::Md />
            <Button label="Large" size=Size::Lg />
            <Button label="Extra Large" size=Size::Xl />
        </div>
    };

    // Style examples
    let style_examples = rsx! {
        <div class="flex flex-wrap gap-4">
            <Button label="Default" />
            <Button label="Outline" style=ButtonStyle::Outline />
            <Button label="Dash" style=ButtonStyle::Dash />
            <Button label="Soft" style=ButtonStyle::Soft />
            <Button label="Ghost" style=ButtonStyle::Ghost />
            <Button label="Link" style=ButtonStyle::Link />
        </div>
    };

    // Modifier examples
    let modifier_examples = rsx! {
        <div class="flex flex-wrap gap-4">
            <Button label="Wide" modifier=ButtonModifier::Wide />
            <Button label="Block" modifier=ButtonModifier::Block />
            <Button label="Square" modifier=ButtonModifier::Square />
            <Button label="Circle" modifier=ButtonModifier::Circle />
        </div>
    };

    // State examples
    let state_examples = rsx! {
        <div class="flex flex-wrap gap-4">
            <Button label="Disabled" disabled=true />
            <Button label="Loading" loading=true />
        </div>
    };

    // Type examples
    let type_examples = rsx! {
        <div class="flex flex-wrap gap-4">
            <Button label="Button (default)" />
            <Button label="Submit" button_type=ButtonType::Submit />
            <Button label="Reset" button_type=ButtonType::Reset />
        </div>
    };

    // Custom class example
    let custom_examples = rsx! {
        <div class="flex flex-wrap gap-4">
            <Button label="Custom class" class="btn-secondary btn-outline" />
            <Button label="With ID" id="my-btn" />
        </div>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Button"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Buttons allow users to take actions and make choices with a single tap."
            </p>

            <div class="divider">"Colors"</div>
            {basic_examples}

            <div class="divider">"Sizes"</div>
            {size_examples}

            <div class="divider">"Styles"</div>
            {style_examples}

            <div class="divider">"Modifiers"</div>
            {modifier_examples}

            <div class="divider">"States"</div>
            {state_examples}

            <div class="divider">"Types"</div>
            {type_examples}

            <div class="divider">"Custom Classes"</div>
            {custom_examples}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Button, Color, Size, ButtonStyle};",
                "use rsx_macros::rsx;",
                "",
                "let html = rsx! {",
                "    <Button label=\"Primary\" color=Color::Primary size=Size::Lg />",
                "    <Button label=\"Outline\" style=ButtonStyle::Outline />",
                "    <Button label=\"Disabled\" disabled=true />",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/button"))
}
