use axum::response::Html;
use rsx::rsx;
use rsxui::components::{Carousel, CarouselDirection, CarouselItem, CarouselSnap};

use crate::layout;

pub async fn page() -> Html<String> {
    let img1 = "https://img.daisyui.com/images/stock/photo-1559703248-dcaaec9fab78.webp";
    let img2 = "https://img.daisyui.com/images/stock/photo-1565098772267-60af42b81ef2.webp";
    let img3 = "https://img.daisyui.com/images/stock/photo-1572635148818-ef6fd45eb394.webp";
    let img4 = "https://img.daisyui.com/images/stock/photo-1494253109108-2e30c049369b.webp";
    let img5 = "https://img.daisyui.com/images/stock/photo-1550258987-190a2d41a8ba.webp";
    let img6 = "https://img.daisyui.com/images/stock/photo-1559181567-c3190ca9959b.webp";

    let basic_example = rsx! {
        <Carousel class="rounded-box" aria_label="Image gallery">
            <CarouselItem><img src={img1} alt="Burger" /></CarouselItem>
            <CarouselItem><img src={img2} alt="Pizza" /></CarouselItem>
            <CarouselItem><img src={img3} alt="Tacos" /></CarouselItem>
            <CarouselItem><img src={img4} alt="Salad" /></CarouselItem>
            <CarouselItem><img src={img5} alt="Pineapple" /></CarouselItem>
            <CarouselItem><img src={img6} alt="Cherry" /></CarouselItem>
        </Carousel>
    };

    let center_example = rsx! {
        <Carousel snap=CarouselSnap::Center class="rounded-box" aria_label="Image gallery center">
            <CarouselItem><img src={img1} alt="Burger" /></CarouselItem>
            <CarouselItem><img src={img2} alt="Pizza" /></CarouselItem>
            <CarouselItem><img src={img3} alt="Tacos" /></CarouselItem>
            <CarouselItem><img src={img4} alt="Salad" /></CarouselItem>
            <CarouselItem><img src={img5} alt="Pineapple" /></CarouselItem>
            <CarouselItem><img src={img6} alt="Cherry" /></CarouselItem>
        </Carousel>
    };

    let end_example = rsx! {
        <Carousel snap=CarouselSnap::End class="rounded-box" aria_label="Image gallery end">
            <CarouselItem><img src={img1} alt="Burger" /></CarouselItem>
            <CarouselItem><img src={img2} alt="Pizza" /></CarouselItem>
            <CarouselItem><img src={img3} alt="Tacos" /></CarouselItem>
            <CarouselItem><img src={img4} alt="Salad" /></CarouselItem>
            <CarouselItem><img src={img5} alt="Pineapple" /></CarouselItem>
            <CarouselItem><img src={img6} alt="Cherry" /></CarouselItem>
        </Carousel>
    };

    let vertical_example = rsx! {
        <Carousel direction=CarouselDirection::Vertical class="rounded-box h-96" aria_label="Vertical gallery">
            <CarouselItem class="h-full"><img src={img1} alt="Burger" /></CarouselItem>
            <CarouselItem class="h-full"><img src={img2} alt="Pizza" /></CarouselItem>
            <CarouselItem class="h-full"><img src={img3} alt="Tacos" /></CarouselItem>
        </Carousel>
    };

    let full_width_example = rsx! {
        <Carousel class="rounded-box w-64" aria_label="Full width gallery">
            <CarouselItem class="w-full"><img src={img1} class="w-full" alt="Burger" /></CarouselItem>
            <CarouselItem class="w-full"><img src={img2} class="w-full" alt="Pizza" /></CarouselItem>
            <CarouselItem class="w-full"><img src={img3} class="w-full" alt="Tacos" /></CarouselItem>
        </Carousel>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Carousel"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Carousel shows images or content in a scrollable area."
            </p>

            <div class="alert alert-info mb-8">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="h-6 w-6 shrink-0 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                <span>"Focus the carousel with "<kbd class="kbd kbd-sm">Tab</kbd>" then use "<kbd class="kbd kbd-sm">Left</kbd>" / "<kbd class="kbd kbd-sm">Right</kbd>" arrow keys to scroll."</span>
            </div>

            <div class="divider">"Snap to Start (Default)"</div>
            {basic_example}

            <div class="divider">"Snap to Center"</div>
            {center_example}

            <div class="divider">"Snap to End"</div>
            {end_example}

            <div class="divider">"Vertical Carousel"</div>
            {vertical_example}

            <div class="divider">"Full Width Items"</div>
            {full_width_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Carousel, CarouselItem, CarouselSnap};",
                "use rsx::rsx;",
                "",
                "let html = rsx! {",
                "    <Carousel snap=CarouselSnap::Center class=\"rounded-box\">",
                "        <CarouselItem><img src=\"1.jpg\" /></CarouselItem>",
                "        <CarouselItem><img src=\"2.jpg\" /></CarouselItem>",
                "    </Carousel>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/carousel").await)
}
