//! Mask Component
//!
//! Mask crops the content of the element to common shapes.
//!
//! # DaisyUI Classes
//! - Base: `mask`
//! - Style: `mask-squircle`, `mask-heart`, `mask-hexagon`, `mask-hexagon-2`, `mask-decagon`, `mask-pentagon`, `mask-diamond`, `mask-square`, `mask-circle`, `mask-star`, `mask-star-2`, `mask-triangle`, `mask-triangle-2`, `mask-triangle-3`, `mask-triangle-4`
//! - Modifier: `mask-half-1`, `mask-half-2`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{Mask, MaskShape};
//! use rsx_macros::rsx;
//!
//! let html = rsx! {
//!     <Mask shape={MaskShape::Circle} src="photo.jpg" />
//! };
//! ```

use enum_stringify::EnumStringify;
use rsx_macros::{classes, component, rsx};

// ============================================
// MaskShape - Mask shape styles
// ============================================

#[derive(Debug, Clone, PartialEq, Default, EnumStringify)]
#[enum_stringify(case = "kebab", prefix = "mask-")]
pub enum MaskShape {
    #[default]
    #[enum_stringify(rename = "")]
    Default,
    Squircle,
    Heart,
    Hexagon,
    #[enum_stringify(rename = "hexagon-2")]
    Hexagon2,
    Decagon,
    Pentagon,
    Diamond,
    Square,
    Circle,
    Star,
    #[enum_stringify(rename = "star-2")]
    Star2,
    Triangle,
    #[enum_stringify(rename = "triangle-2")]
    Triangle2,
    #[enum_stringify(rename = "triangle-3")]
    Triangle3,
    #[enum_stringify(rename = "triangle-4")]
    Triangle4,
}

// ============================================
// MaskModifier - Mask half modifiers
// ============================================

#[derive(Debug, Clone, PartialEq, Default, EnumStringify)]
#[enum_stringify(case = "kebab", prefix = "mask-")]
pub enum MaskModifier {
    #[default]
    #[enum_stringify(rename = "")]
    Default,
    Half1,
    Half2,
}

// ============================================
// Mask - Shape mask wrapper
// ============================================

#[component]
pub fn Mask(
    shape: MaskShape,
    #[builder(default)] modifier: MaskModifier,
    #[builder(default)] class: String,
    children: String,
) -> String {
    rsx! {
        <div class={classes!("mask", shape.to_string(), modifier.to_string(), class)}>
            {children}
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

    #[tokio::test]
    async fn test_mask_basic() {
        let html = rsx! {
            <Mask shape=MaskShape::Circle>
                <img src="photo.jpg" />
            </Mask>
        };
        assert!(html.contains(r#"class="mask mask-circle""#));
        assert!(html.contains("<img"));
    }

    #[tokio::test]
    async fn test_mask_all_shapes() {
        for shape in [
            MaskShape::Squircle,
            MaskShape::Heart,
            MaskShape::Hexagon,
            MaskShape::Hexagon2,
            MaskShape::Decagon,
            MaskShape::Pentagon,
            MaskShape::Diamond,
            MaskShape::Square,
            MaskShape::Circle,
            MaskShape::Star,
            MaskShape::Star2,
            MaskShape::Triangle,
            MaskShape::Triangle2,
            MaskShape::Triangle3,
            MaskShape::Triangle4,
        ] {
            let html = rsx! {
                <Mask shape=shape.clone()>"X"</Mask>
            };
            let expected = shape.to_string();
            assert!(
                html.contains(&expected),
                "Missing shape class {} for {:?}",
                expected,
                shape
            );
        }
    }

    #[tokio::test]
    async fn test_mask_modifier() {
        let html = rsx! {
            <Mask shape=MaskShape::Circle modifier=MaskModifier::Half1>
                "X"
            </Mask>
        };
        assert!(html.contains("mask-half-1"));
    }

    #[tokio::test]
    async fn test_mask_custom_class() {
        let html = rsx! {
            <Mask shape=MaskShape::Heart class="w-24">
                "X"
            </Mask>
        };
        assert!(html.contains("w-24"));
    }

    #[tokio::test]
    async fn test_mask_default_no_extra_classes() {
        let html = rsx! {
            <Mask shape=MaskShape::Square>"X"</Mask>
        };
        assert!(!html.contains("mask-half-1"));
        assert!(!html.contains("mask-half-2"));
    }
}
