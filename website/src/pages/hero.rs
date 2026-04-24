use axum::response::Html;
use rsx_macros::rsx;
use rsxui::components::{Hero, HeroContent};

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <Hero class="bg-base-200 min-h-[300px] rounded-box">
            <HeroContent class="text-center">
                <div class="max-w-md">
                    <h1 class="text-5xl font-bold">"Hello there"</h1>
                    <p class="py-6">"Provident cupiditate voluptatem et in. Quaerat fugiat ut assumenda excepturi exercitationem quasi."</p>
                    <button class="btn btn-primary">"Get Started"</button>
                </div>
            </HeroContent>
        </Hero>
    };

    let image_example = rsx! {
        <Hero class="min-h-[300px] rounded-box" style="background-image: url(https://img.daisyui.com/images/stock/photo-1507358522600-9f71e620c44e.webp);">
            <div class="hero-overlay bg-opacity-60 rounded-box" />
            <HeroContent class="text-center text-neutral-content">
                <div class="max-w-md">
                    <h1 class="mb-5 text-5xl font-bold">"Hello there"</h1>
                    <p class="mb-5">"Provident cupiditate voluptatem et in."</p>
                    <button class="btn btn-primary">"Get Started"</button>
                </div>
            </HeroContent>
        </Hero>
    };

    let side_example = rsx! {
        <Hero class="bg-base-200 min-h-[300px] rounded-box">
            <HeroContent>
                <img
                    src="https://img.daisyui.com/images/stock/photo-1635805737707-575885ab0820.webp"
                    class="max-w-sm rounded-lg shadow-2xl"
                />
                <div>
                    <h1 class="text-5xl font-bold">"Spider-Man!"</h1>
                    <p class="py-6">"Spider-Man is a superhero appearing in American comic books published by Marvel Comics."</p>
                    <button class="btn btn-primary">"Watch Movie"</button>
                </div>
            </HeroContent>
        </Hero>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Hero"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Hero is a component for displaying a large box or image with a title and description."
            </p>

            <div class="divider">"Basic Hero"</div>
            {basic_example}

            <div class="divider">"Hero with Background Image"</div>
            <p class="text-sm text-gray-600 mb-4">"Use hero-overlay for a tinted overlay."</p>
            {image_example}

            <div class="divider">"Hero with Side Content"</div>
            <p class="text-sm text-gray-600 mb-4">"Image and text side by side."</p>
            {side_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Hero, HeroContent};",
                "use rsx_macros::rsx;",
                "",
                "let html = rsx! {",
                "    <Hero class=\"bg-base-200 min-h-[300px]\">",
                "        <HeroContent class=\"text-center\">",
                "            <h1 class=\"text-5xl font-bold\">\"Hello\"</h1>",
                "            <p>\"Welcome to our site.\"</p>",
                "            <button class=\"btn btn-primary\">\"Get Started\"</button>",
                "        </HeroContent>",
                "    </Hero>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/hero").await)
}
