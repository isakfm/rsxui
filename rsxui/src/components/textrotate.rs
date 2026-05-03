//! TextRotate Component
//!
//! Text Rotate can show up to 6 lines of text, one at a time, with an infinite loop animation.
//! Duration is 10 seconds by default. The animation will pause on hover.
//!
//! # DaisyUI Classes
//! - Base: `text-rotate`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::TextRotate;
//! use rsx::rsx;
//!
//! let html = rsx! {
//!     <TextRotate>
//!         <span>"ONE"</span>
//!         <span>"TWO"</span>
//!         <span>"THREE"</span>
//!     </TextRotate>
//! };
//! ```

use rsx::attrs::RenderAttrs;
use rsx::{classes, rsx, ui};

use super::class_if;

// ============================================
// TextRotate - Rotating text animation
// ============================================

#[ui]
pub fn TextRotate(
    #[builder(default)] center: bool,
    #[builder(default)] class: String,
    children: String,
) -> String {
    rsx! {
        <span class={classes!("text-rotate", props.class)} {props.render_attrs()}>
            <span class={classes!(class_if(props.center, "justify-items-center"))}>
                {props.children}
            </span>
        </span>
    }
}

// ============================================
// Tests
// ============================================

#[cfg(test)]
mod tests {
    use super::*;
    use rsx::rsx;

    #[tokio::test]
    async fn test_text_rotate_basic() {
        let html = rsx! {
            <TextRotate>
                <span>"ONE"</span>
                <span>"TWO"</span>
                <span>"THREE"</span>
            </TextRotate>
        };
        assert!(html.contains(r#"class="text-rotate""#));
        assert!(html.contains(">ONE</span>"));
        assert!(html.contains(">TWO</span>"));
        assert!(html.contains(">THREE</span>"));
    }

    #[tokio::test]
    async fn test_text_rotate_centered() {
        let html = rsx! {
            <TextRotate center=true>
                <span>"A"</span>
            </TextRotate>
        };
        assert!(html.contains("justify-items-center"));
    }

    #[tokio::test]
    async fn test_text_rotate_custom_class() {
        let html = rsx! {
            <TextRotate class="text-7xl">
                <span>"A"</span>
            </TextRotate>
        };
        assert!(html.contains("text-7xl"));
    }
}
