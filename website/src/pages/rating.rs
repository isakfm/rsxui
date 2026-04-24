use axum::response::Html;
use rsx_macros::rsx;
use rsxui::components::{Rating, Size};

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <Rating max=5 checked=3 />
    };

    let readonly_example = rsx! {
        <Rating max=5 checked=3 read_only=true />
    };

    let star2_example = rsx! {
        <Rating max=5 checked=2 mask="mask-star-2" color="bg-orange-400" />
    };

    let sizes_example = rsx! {
        <div class="flex flex-col gap-4 items-center">
            <Rating max=5 checked=2 mask="mask-star-2" color="bg-orange-400" size=Size::Xs />
            <Rating max=5 checked=2 mask="mask-star-2" color="bg-orange-400" size=Size::Sm />
            <Rating max=5 checked=2 mask="mask-star-2" color="bg-orange-400" size=Size::Md />
            <Rating max=5 checked=2 mask="mask-star-2" color="bg-orange-400" size=Size::Lg />
            <Rating max=5 checked=2 mask="mask-star-2" color="bg-orange-400" size=Size::Xl />
        </div>
    };

    let clearable_example = rsx! {
        <Rating max=5 checked=2 mask="mask-star-2" size=Size::Lg hidden=true />
    };

    let half_example = rsx! {
        <Rating max=5 checked=3 half=true mask="mask-star-2" color="bg-green-500" size=Size::Lg hidden=true />
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Rating"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Rating is a set of radio buttons that allow the user to rate something."
            </p>

            <div class="divider">"Basic Rating"</div>
            <p class="text-sm text-gray-600 mb-4">"Click a star to select a rating."</p>
            {basic_example}

            <div class="divider">"Read Only"</div>
            <p class="text-sm text-gray-600 mb-4">"Use div elements instead of radio inputs for display-only ratings."</p>
            {readonly_example}

            <div class="divider">"mask-star-2 with Color"</div>
            <p class="text-sm text-gray-600 mb-4">"Different mask shape and background color."</p>
            {star2_example}

            <div class="divider">"Sizes"</div>
            <p class="text-sm text-gray-600 mb-4">"From extra small to extra large."</p>
            {sizes_example}

            <div class="divider">"Clearable Rating"</div>
            <p class="text-sm text-gray-600 mb-4">"A hidden radio at the start lets users clear their rating."</p>
            {clearable_example}

            <div class="divider">"Half Stars"</div>
            <p class="text-sm text-gray-600 mb-4">"Allow half-star ratings with mask-half-1 and mask-half-2."</p>
            {half_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Rating, Size};",
                "use rsx_macros::rsx;",
                "",
                "// Basic rating with checked state",
                "let html = rsx! {",
                "    <Rating max={5} checked={3} />",
                "};",
                "",
                "// Read-only display",
                "let html = rsx! {",
                "    <Rating max={5} checked={3} read_only={true} />",
                "};",
                "",
                "// Half stars with custom mask and color",
                "let html = rsx! {",
                "    <Rating",
                "        max={5}",
                "        half={true}",
                "        checked={3}",
                "        mask=\"mask-star-2\"",
                "        color=\"bg-green-500\"",
                "        size={Size::Lg}",
                "        hidden={true}",
                "    />",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/rating").await)
}
