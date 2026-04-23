//! Badge Component
//!
//! A badge component with DaisyUI styling.
//!
//! # DaisyUI Classes
//! - Base: `badge`
//! - Colors: `badge-primary`, `badge-secondary`, etc.
//! - Sizes: `badge-xs`, `badge-sm`, `badge-md`, `badge-lg`, `badge-xl`
//! - Styles: `badge-outline`, `badge-dash`, `badge-soft`, `badge-ghost`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{Badge, Color, Size};
//! use rsx_macros::rsx;
//!
//! let html = rsx! {
//!     <Badge text="New" color=Color::Primary size=Size::Sm />
//! };
//! ```

use rsx::attrs::RenderAttrs;
use rsx_macros::{classes, rsx, ui};
use enum_stringify::EnumStringify;

use super::{Color, Size};

#[derive(Debug, Clone, PartialEq, Default, EnumStringify)]
#[enum_stringify(case = "kebab", prefix = "badge-")]
pub enum BadgeStyle {
    #[default]
    #[enum_stringify(rename = "")]
    Default,
    Outline,
    Dash,
    Soft,
    Ghost,
}

#[ui]
pub fn Badge(
    text: String,
    color: Color,
    size: Size,
    #[builder(default)] style: BadgeStyle,
    #[builder(default)] class: String,
) -> String {
    rsx! {
        <span
            class={classes!(
                "badge",
                props.color.prefix("badge"),
                props.size.prefix("badge"),
                props.style,
                props.class,
            )}{props.render_attrs()}>{props.text}</span>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rsx_macros::rsx;

    #[tokio::test]
    async fn test_badge_basic() {
        let html = rsx! {
            <Badge text="New" />
        };
        assert!(html.contains("<span"));
        assert!(html.contains("class=\"badge\""));
        assert!(html.contains(">New</span>"));
    }

    #[tokio::test]
    async fn test_badge_with_color() {
        let html = rsx! {
            <Badge text="Primary" color=Color::Primary />
        };
        assert!(html.contains("badge-primary"));
    }

    #[tokio::test]
    async fn test_badge_with_size() {
        let html = rsx! {
            <Badge text="Small" size=Size::Sm />
        };
        assert!(html.contains("badge-sm"));
    }

    #[tokio::test]
    async fn test_badge_with_style() {
        let html = rsx! {
            <Badge text="Outline" style=BadgeStyle::Outline />
        };
        assert!(html.contains("badge-outline"));
    }

    #[tokio::test]
    async fn test_badge_with_custom_class() {
        let html = rsx! {
            <Badge text="Custom" class="my-badge" />
        };
        assert!(html.contains("my-badge"));
    }
}
