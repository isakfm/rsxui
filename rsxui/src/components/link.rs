//! Link Component
//!
//! Link adds the missing underline style to links.
//!
//! # DaisyUI Classes
//! - Base: `link`
//! - Style: `link-hover`
//! - Color: `link-neutral`, `link-primary`, `link-secondary`, `link-accent`, `link-success`, `link-info`, `link-warning`, `link-error`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{Link, LinkStyle, Color};
//! use rsx_macros::rsx;
//!
//! let html = rsx! {
//!     <Link href="https://example.com" color={Color::Primary}>
//!         "Click me"
//!     </Link>
//! };
//! ```

use enum_stringify::EnumStringify;
use rsx::attrs::RenderAttrs;
use rsx_macros::{classes, rsx, ui};

use super::Color;

// ============================================
// LinkStyle - Link style variants
// ============================================

#[derive(Debug, Clone, PartialEq, Default, EnumStringify)]
#[enum_stringify(case = "kebab", prefix = "link-")]
pub enum LinkStyle {
    #[default]
    #[enum_stringify(rename = "")]
    Default,
    Hover,
}

// ============================================
// Link - Styled anchor link
// ============================================

#[ui]
pub fn Link(
    href: String,
    #[builder(default)] style: LinkStyle,
    #[builder(default)] color: Color,
    #[builder(default)] class: String,
    children: String,
) -> String {
    rsx! {
        <a
            href={props.href.clone()}
            class={classes!("link", props.style, props.color.prefix("link"), props.class)}
            {props.render_attrs()}>
            {props.children}
        </a>
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
    async fn test_link_basic() {
        let html = rsx! {
            <Link href="https://example.com">"Click me"</Link>
        };
        assert!(html.contains("<a"));
        assert!(html.contains(r#"href="https://example.com""#));
        assert!(html.contains(r#"class="link""#));
        assert!(html.contains(">Click me</a>"));
    }

    #[tokio::test]
    async fn test_link_with_style() {
        let html = rsx! {
            <Link href="/" style=LinkStyle::Hover>"Hover"</Link>
        };
        assert!(html.contains("link-hover"));
    }

    #[tokio::test]
    async fn test_link_all_colors() {
        for color in [
            Color::Primary,
            Color::Secondary,
            Color::Accent,
            Color::Neutral,
            Color::Info,
            Color::Success,
            Color::Warning,
            Color::Error,
        ] {
            let html = rsx! {
                <Link href="/" color=color>"Test"</Link>
            };
            let expected = color.prefix("link");
            assert!(
                html.contains(&expected),
                "Missing color class {} for {:?}",
                expected,
                color
            );
        }
    }

    #[tokio::test]
    async fn test_link_default_no_extra_classes() {
        let html = rsx! {
            <Link href="/">"Default"</Link>
        };
        assert!(!html.contains("link-hover"));
        assert!(!html.contains("link-primary"));
    }

    #[tokio::test]
    async fn test_link_custom_class() {
        let html = rsx! {
            <Link href="/" class="font-bold">"Bold"</Link>
        };
        assert!(html.contains("font-bold"));
    }

    #[tokio::test]
    async fn test_link_with_id() {
        let html = rsx! {
            <Link href="/" id="my-link">"ID"</Link>
        };
        assert!(html.contains(r#"id="my-link""#));
    }

    #[tokio::test]
    async fn test_link_combined() {
        let html = rsx! {
            <Link href="/" style=LinkStyle::Hover color=Color::Primary class="text-lg">
                "Combined"
            </Link>
        };
        assert!(html.contains("link"));
        assert!(html.contains("link-hover"));
        assert!(html.contains("link-primary"));
        assert!(html.contains("text-lg"));
    }
}
