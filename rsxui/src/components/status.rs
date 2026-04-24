//! Status Component
//!
//! A small icon to visually show the current status of an element.
//!
//! # DaisyUI Classes
//! - Base: `status`
//! - Colors: `status-neutral`, `status-primary`, `status-secondary`, `status-accent`, `status-info`, `status-success`, `status-warning`, `status-error`
//! - Sizes: `status-xs`, `status-sm`, `status-md`, `status-lg`, `status-xl`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{Status, Color, Size};
//! use rsx_macros::rsx;
//!
//! let html = rsx! {
//!     <Status color=Color::Success size=Size::Md aria_label="Online" />
//! };
//! ```

use rsx_macros::{classes, component, rsx};

use super::{Color, Size};

// ============================================
// Status - Status indicator dot
// ============================================

#[component]
pub fn Status(
    #[builder(default)] color: Color,
    #[builder(default)] size: Size,
    #[builder(default)] animate: String,
    #[builder(into)] aria_label: Option<String>,
    #[builder(default)] class: String,
) -> String {
    let anim_class = match animate.as_str() {
        "ping" => "animate-ping",
        "bounce" => "animate-bounce",
        _ => "",
    };

    rsx! {
        <div
            class={classes!("status", color.prefix("status"), size.prefix("status"), anim_class, class)}
            {super::attr_if("aria-label", &aria_label)}
        />
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
    async fn test_status_basic() {
        let html = rsx! {
            <Status />
        };
        assert!(html.contains(r#"class="status""#));
    }

    #[tokio::test]
    async fn test_status_all_colors() {
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
                <Status color=color />
            };
            let expected = color.prefix("status");
            assert!(
                html.contains(&expected),
                "Missing color class {} for {:?}",
                expected,
                color
            );
        }
    }

    #[tokio::test]
    async fn test_status_all_sizes() {
        for size in [Size::Xs, Size::Sm, Size::Md, Size::Lg, Size::Xl] {
            let html = rsx! {
                <Status size=size />
            };
            let expected = size.prefix("status");
            assert!(
                html.contains(&expected),
                "Missing size class {} for {:?}",
                expected,
                size
            );
        }
    }

    #[tokio::test]
    async fn test_status_aria_label() {
        let html = rsx! {
            <Status aria_label="Online" />
        };
        assert!(html.contains(r#"aria-label="Online""#));
    }

    #[tokio::test]
    async fn test_status_ping_animation() {
        let html = rsx! {
            <Status animate="ping" />
        };
        assert!(html.contains("animate-ping"));
    }

    #[tokio::test]
    async fn test_status_bounce_animation() {
        let html = rsx! {
            <Status animate="bounce" />
        };
        assert!(html.contains("animate-bounce"));
    }

    #[tokio::test]
    async fn test_status_custom_class() {
        let html = rsx! {
            <Status class="my-status" />
        };
        assert!(html.contains("my-status"));
    }
}
