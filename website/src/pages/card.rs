use axum::response::Html;
use rsx_macros::rsx;
use rsxui::components::{
    Card, CardActions, CardBody, CardFigure, CardLayout, CardStyle, CardTitle, Color, Size,
};

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <Card class="bg-base-100 w-96 shadow-sm">
            <CardBody>
                <CardTitle>"Basic Card"</CardTitle>
                <p>"This is a simple card with just a title and some text content."</p>
            </CardBody>
        </Card>
    };

    let styled_example = rsx! {
        <Card color=Color::Primary style=CardStyle::Border class="bg-base-100 w-96 shadow-sm">
            <CardBody>
                <CardTitle>"Styled Card"</CardTitle>
                <p>"This card uses primary color with border style."</p>
            </CardBody>
        </Card>
    };

    let with_image = rsx! {
        <Card style=CardStyle::Border class="bg-base-100 w-96 shadow-sm">
            <CardFigure>
                <img src="https://img.daisyui.com/images/stock/photo-1606107557195-0e29a4b5b4aa.webp" alt="Shoes" />
            </CardFigure>
            <CardBody>
                <CardTitle>"Card with Image"</CardTitle>
                <p>"Cards can include a figure element for images."</p>
                <CardActions class="justify-end">
                    <button class="btn btn-primary">Buy Now</button>
                </CardActions>
            </CardBody>
        </Card>
    };

    let side_layout = rsx! {
        <Card layout=CardLayout::Side style=CardStyle::Border class="bg-base-100 w-96 shadow-sm">
            <CardFigure>
                <img src="https://img.daisyui.com/images/stock/photo-1635805737707-575885ab0820.webp" alt="Movie" />
            </CardFigure>
            <CardBody>
                <CardTitle>"Side Layout"</CardTitle>
                <p>"The side layout places the image next to the content."</p>
                <CardActions class="justify-end">
                    <button class="btn btn-primary">Watch</button>
                </CardActions>
            </CardBody>
        </Card>
    };

    let sized_example = rsx! {
        <div class="flex flex-col gap-4">
            <Card size=Size::Xs class="bg-base-100 w-96 shadow-sm">
                <CardBody>
                    <CardTitle>"Extra Small"</CardTitle>
                    <p>"Extra small card."</p>
                </CardBody>
            </Card>
            <Card size=Size::Sm class="bg-base-100 w-96 shadow-sm">
                <CardBody>
                    <CardTitle>"Small"</CardTitle>
                    <p>"Small card."</p>
                </CardBody>
            </Card>
            <Card size=Size::Md class="bg-base-100 w-96 shadow-sm">
                <CardBody>
                    <CardTitle>"Medium"</CardTitle>
                    <p>"Medium card (default)."</p>
                </CardBody>
            </Card>
            <Card size=Size::Lg class="bg-base-100 w-96 shadow-sm">
                <CardBody>
                    <CardTitle>"Large"</CardTitle>
                    <p>"Large card."</p>
                </CardBody>
            </Card>
        </div>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Card"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Cards are used to group and display content in a way that is easily readable."
            </p>

            <div class="divider">"Basic Card"</div>
            <div class="max-w-sm">
                {basic_example}
            </div>

            <div class="divider">"Styled Card"</div>
            <div class="max-w-sm">
                {styled_example}
            </div>

            <div class="divider">"With Image"</div>
            <div class="max-w-sm">
                {with_image}
            </div>

            <div class="divider">"Side Layout"</div>
            <div class="max-w-2xl">
                {side_layout}
            </div>

            <div class="divider">"Sizes"</div>
            <div class="max-w-sm">
                {sized_example}
            </div>

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{",
                "    Card, CardBody, CardFigure, CardTitle, CardActions,",
                "    CardStyle, Color",
                "};",
                "use rsx_macros::rsx;",
                "",
                "let html = rsx! {",
                "    <Card color=Color::Primary style=CardStyle::Border class=\"bg-base-100 w-96 shadow-sm\">",
                "        <CardFigure>",
                "            <img src=\"photo.jpg\" />",
                "        </CardFigure>",
                "        <CardBody>",
                "            <CardTitle>\"Title\"</CardTitle>",
                "            <p>\"Content\"</p>",
                "            <CardActions>",
                "                <button class=\"btn\">Action</button>",
                "            </CardActions>",
                "        </CardBody>",
                "    </Card>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/card"))
}
