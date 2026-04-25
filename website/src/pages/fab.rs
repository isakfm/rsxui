use axum::response::Html;
use rsx_macros::rsx;
use rsxui::components::{Fab, FabClose, FabMainAction, FabModifier};

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <div class="h-54 relative">
            <Fab class="absolute z-1">
                <div tabindex="0" role="button" class="btn btn-lg btn-circle btn-primary">"F"</div>
                <button class="btn btn-lg btn-circle">"A"</button>
                <button class="btn btn-lg btn-circle">"B"</button>
                <button class="btn btn-lg btn-circle">"C"</button>
            </Fab>
        </div>
    };

    let with_labels_example = rsx! {
        <div class="h-54 relative">
            <Fab class="absolute z-1">
                <div tabindex="0" role="button" class="btn btn-lg btn-circle btn-success">"F"</div>
                <div>"Label B" <button class="btn btn-lg btn-circle">"A"</button></div>
                <div>"Label C" <button class="btn btn-lg btn-circle">"B"</button></div>
                <div>"Label D" <button class="btn btn-lg btn-circle">"C"</button></div>
            </Fab>
        </div>
    };

    let with_close_example = rsx! {
        <div class="h-54 relative">
            <Fab class="absolute z-1">
                <div tabindex="0" role="button" class="btn btn-lg btn-circle btn-info">"F"</div>
                <FabClose>
                    "Close" <span class="btn btn-circle btn-lg btn-error">"X"</span>
                </FabClose>
                <div>"Label A" <button class="btn btn-lg btn-circle">"A"</button></div>
                <div>"Label B" <button class="btn btn-lg btn-circle">"B"</button></div>
            </Fab>
        </div>
    };

    let with_main_action_example = rsx! {
        <div class="h-54 relative">
            <Fab class="absolute z-1">
                <div tabindex="0" role="button" class="btn btn-lg btn-circle btn-primary">"F"</div>
                <FabMainAction>
                    "Main Action" <button class="btn btn-circle btn-secondary btn-lg">"M"</button>
                </FabMainAction>
                <div>"Label A" <button class="btn btn-lg btn-circle">"A"</button></div>
                <div>"Label B" <button class="btn btn-lg btn-circle">"B"</button></div>
            </Fab>
        </div>
    };

    let flower_example = rsx! {
        <div class="h-54 relative">
            <Fab modifier=FabModifier::Flower class="absolute z-1">
                <div tabindex="0" role="button" class="btn btn-lg btn-circle btn-success">"F"</div>
                <button class="fab-main-action btn btn-circle btn-lg">"M"</button>
                <button class="btn btn-lg btn-circle">"A"</button>
                <button class="btn btn-lg btn-circle">"B"</button>
                <button class="btn btn-lg btn-circle">"C"</button>
            </Fab>
        </div>
    };

    let single_fab_example = rsx! {
        <div class="h-54 relative">
            <Fab class="absolute z-1">
                <button class="btn btn-lg btn-circle btn-primary">"F"</button>
            </Fab>
        </div>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"FAB / Speed Dial"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "FAB (Floating Action Button) stays in the bottom corner of screen. Clicking or focusing it shows additional buttons (Speed Dial)."
            </p>

            <div class="divider">"Basic FAB"</div>
            {basic_example}

            <div class="divider">"Single FAB"</div>
            {single_fab_example}

            <div class="divider">"With Labels"</div>
            {with_labels_example}

            <div class="divider">"With Close Button"</div>
            {with_close_example}

            <div class="divider">"With Main Action"</div>
            {with_main_action_example}

            <div class="divider">"Flower Arrangement"</div>
            {flower_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Fab, FabModifier};",
                "use rsx_macros::rsx;",
                "",
                "let html = rsx! {",
                "    <Fab modifier=FabModifier::Flower>",
                "        <div tabindex=\"0\" role=\"button\" class=\"btn btn-circle\">\"F\"</div>",
                "        <button class=\"btn btn-circle\">\"A\"</button>",
                "    </Fab>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/fab").await)
}
