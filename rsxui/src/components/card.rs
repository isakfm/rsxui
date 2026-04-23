//! Card Component
//!
//! A flexible, composable card component with DaisyUI styling.
//!
//! Compose cards using sub-components for full control over structure.
//!
//! # DaisyUI Classes
//! - Base: `card`
//! - Sizes: `card-xs`, `card-sm`, `card-md`, `card-lg`, `card-xl`
//! - Styles: `card-border`, `card-dash`
//! - Layouts: `card-side`, `image-full`
//! - Parts: `card-body`, `card-title`, `card-actions`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{Card, CardBody, CardFigure, CardTitle, CardActions, CardStyle, Color, Size};
//! use rsx_macros::rsx;
//!
//! let html = rsx! {
//!     <Card color=Color::Primary size=Size::Md style=CardStyle::Border>
//!         <CardFigure>
//!             <img src="photo.jpg" alt="Photo" />
//!         </CardFigure>
//!         <CardBody>
//!             <CardTitle>"Product Name"</CardTitle>
//!             "Description text"
//!             <CardActions class="justify-end">
//!                 <button class="btn btn-primary">Buy Now</button>
//!             </CardActions>
//!         </CardBody>
//!     </Card>
//! };
//! ```

use rsx::attrs::RenderAttrs;
use rsx_macros::{classes, rsx, ui};
use enum_stringify::EnumStringify;

use super::{Color, Size};

// ============================================
// CardStyle - Border variations
// ============================================

#[derive(Debug, Clone, PartialEq, Default, EnumStringify)]
#[enum_stringify(case = "kebab", prefix = "card-")]
pub enum CardStyle {
    #[default]
    #[enum_stringify(rename = "")]
    Default,
    Border, // card-border
    Dash,   // card-dash
}

// ============================================
// CardLayout - Layout modifiers
// ============================================

#[derive(Debug, Clone, PartialEq, Default, EnumStringify)]
pub enum CardLayout {
    #[default]
    #[enum_stringify(rename = "")]
    Default,
    #[enum_stringify(rename = "card-side")]
    Side,
    #[enum_stringify(rename = "image-full")]
    ImageFull,
}

// ============================================
// Card - Outer wrapper
// ============================================

#[ui]
pub fn Card(
    color: Color,
    size: Size,
    #[builder(default)] style: CardStyle,
    #[builder(default)] layout: CardLayout,
    #[builder(default)] class: String,
    children: String,
) -> String {
    rsx! {
        <div
            class={classes!(
                "card",
                props.color.prefix("card"),
                props.size.prefix("card"),
                props.style,
                props.layout,
                props.class,
            )}
            {props.render_attrs()}>
            {props.children}
        </div>
    }
}

// ============================================
// CardFigure - Image/media wrapper
// ============================================

#[ui]
pub fn CardFigure(
    #[builder(default)] class: String,
    children: String,
) -> String {
    rsx! {
        <figure class={classes!(props.class)}>
            {props.children}
        </figure>
    }
}

// ============================================
// CardBody - Content container
// ============================================

#[ui]
pub fn CardBody(
    #[builder(default)] class: String,
    children: String,
) -> String {
    rsx! {
        <div class={classes!("card-body", props.class)}>
            {props.children}
        </div>
    }
}

// ============================================
// CardTitle - Title heading
// ============================================

#[ui]
pub fn CardTitle(
    #[builder(default)] class: String,
    children: String,
) -> String {
    rsx! {
        <h2 class={classes!("card-title", props.class)}>
            {props.children}
        </h2>
    }
}

// ============================================
// CardActions - Action buttons container
// ============================================

#[ui]
pub fn CardActions(
    #[builder(default)] class: String,
    children: String,
) -> String {
    rsx! {
        <div class={classes!("card-actions", props.class)}>
            {props.children}
        </div>
    }
}

// ============================================
// Tests
// ============================================

#[cfg(test)]
mod tests {
    use super::*;
    use rsx_macros::rsx;
    use crate::components::Button;

    #[tokio::test]
    async fn test_card_basic() {
        let html = rsx! {
            <Card>"Hello"</Card>
        };
        assert!(html.contains("<div"));
        assert!(html.contains("class=\"card\""));
        assert!(html.contains(">Hello</div>"));
    }

    #[tokio::test]
    async fn test_card_composition_full() {
        let html = rsx! {
            <Card color=Color::Primary size=Size::Md style=CardStyle::Border>
                <CardFigure>
                    <img src="photo.jpg" alt="Photo" />
                </CardFigure>
                <CardBody>
                    <CardTitle>"Product Name"</CardTitle>
                    "Description text"
                    <CardActions class="justify-end">
                    <Button color=Color::Primary label="Buy" />
                    </CardActions>
                </CardBody>
            </Card>
        };
        assert!(html.contains("card"));
        assert!(html.contains("card-primary"));
        assert!(html.contains("card-border"));
        assert!(html.contains("<figure"));
        assert!(html.contains("<img"));
        assert!(html.contains("card-body"));
        assert!(html.contains("card-title"));
        assert!(html.contains("card-actions"));
        assert!(html.contains("justify-end"));
        assert!(html.contains("Buy</button>"));
    }

    #[tokio::test]
    async fn test_card_body_only() {
        let html = rsx! {
            <Card>
                <CardBody>"Just body"</CardBody>
            </Card>
        };
        assert!(html.contains("class=\"card\""));
        assert!(html.contains("card-body"));
        assert!(html.contains("Just body"));
        assert!(!html.contains("<figure>"));
        assert!(!html.contains("card-title"));
        assert!(!html.contains("card-actions"));
    }

    #[tokio::test]
    async fn test_card_with_figure_and_body() {
        let html = rsx! {
            <Card>
                <CardFigure>
                    <img src="pic.jpg" />
                </CardFigure>
                <CardBody>"With image"</CardBody>
            </Card>
        };
        assert!(html.contains("<figure"));
        assert!(html.contains("<img"));
        assert!(html.contains("card-body"));
        assert!(html.contains("With image"));
    }

    #[tokio::test]
    async fn test_card_with_title_and_actions() {
        let html = rsx! {
            <Card>
                <CardBody>
                    <CardTitle>"My Title"</CardTitle>
                    "Content"
                    <CardActions>
                        <Button label="Action" />
                    </CardActions>
                </CardBody>
            </Card>
        };
        assert!(html.contains("card-title"));
        assert!(html.contains(">My Title</h2>"));
        assert!(html.contains("card-actions"));
        assert!(html.contains("Action</button>"));
        assert!(html.contains("Content"));
    }

    #[tokio::test]
    async fn test_card_body_custom_class() {
        let html = rsx! {
            <Card>
                <CardBody class="items-center text-center">
                    "Centered"
                </CardBody>
            </Card>
        };
        assert!(html.contains("card-body items-center text-center"));
    }

    #[tokio::test]
    async fn test_card_actions_justify_end() {
        let html = rsx! {
            <Card>
                <CardBody>
                    <CardActions class="justify-end">
                        <Button label="Go" />
                    </CardActions>
                </CardBody>
            </Card>
        };
        assert!(html.contains("card-actions justify-end"));
    }

    #[tokio::test]
    async fn test_card_with_color() {
        let html = rsx! {
            <Card color=Color::Primary>"Primary"</Card>
        };
        assert!(html.contains("card-primary"));
    }

    #[tokio::test]
    async fn test_card_border_style() {
        let html = rsx! {
            <Card style=CardStyle::Border>"Bordered"</Card>
        };
        assert!(html.contains("card-border"));
    }

    #[tokio::test]
    async fn test_card_dash_style() {
        let html = rsx! {
            <Card style=CardStyle::Dash>"Dashed"</Card>
        };
        assert!(html.contains("card-dash"));
    }

    #[tokio::test]
    async fn test_card_side_layout() {
        let html = rsx! {
            <Card layout=CardLayout::Side>"Side"</Card>
        };
        assert!(html.contains("card-side"));
    }

    #[tokio::test]
    async fn test_card_image_full_layout() {
        let html = rsx! {
            <Card layout=CardLayout::ImageFull>"Full"</Card>
        };
        assert!(html.contains("image-full"));
    }

    #[tokio::test]
    async fn test_card_default_no_extra_classes() {
        let html = rsx! {
            <Card>"Plain"</Card>
        };
        assert!(!html.contains("card-border"));
        assert!(!html.contains("card-dash"));
        assert!(!html.contains("card-side"));
        assert!(!html.contains("image-full"));
    }

    #[tokio::test]
    async fn test_card_id() {
        let html = rsx! {
            <Card id="first">"First"</Card>
        };
        assert!(html.contains("id=\"first\""));
    }
}
