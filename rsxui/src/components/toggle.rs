//! Toggle Component
//!
//! A toggle (styled checkbox) component with DaisyUI styling.
//!
//! # DaisyUI Classes
//! - Base: `toggle`
//! - Colors: `toggle-primary`, `toggle-secondary`, etc.
//! - Sizes: `toggle-xs`, `toggle-sm`, `toggle-md`, `toggle-lg`, `toggle-xl`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{Toggle, Color, Size};
//! use rsx::rsx;
//!
//! let html = rsx! {
//!     <Toggle color={Color::Primary} size={Size::Lg} checked=true />
//! };
//! ```

use rsx::{classes, component, rsx};

use super::{attr_if, show_if, Color, Size};

// ============================================
// Toggle
// ============================================

#[component]
pub fn Toggle(
    #[builder(into)] name: Option<String>,
    #[builder(into)] value: Option<String>,
    #[builder(default)] color: Color,
    #[builder(default)] size: Size,
    #[builder(default)] checked: bool,
    #[builder(default)] disabled: bool,
    #[builder(default)] class: String,
) -> String {
    rsx! {
        <input
            type="checkbox"
            class={classes!(
                "toggle",
                color.prefix("toggle"),
                size.prefix("toggle"),
                class,
            )}
            {show_if(checked, "checked")}
            {show_if(disabled, "disabled")}
            {attr_if("name", &name)}
            {attr_if("value", &value)}
        />
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
    async fn test_toggle_basic() {
        let html = rsx! {
            <Toggle />
        };
        assert!(html.contains("<input"));
        assert!(html.contains(r#"type="checkbox""#));
        assert!(html.contains("class=\"toggle\""));
    }

    #[tokio::test]
    async fn test_toggle_with_color() {
        let html = rsx! {
            <Toggle color=Color::Primary />
        };
        assert!(html.contains("toggle-primary"));
    }

    #[tokio::test]
    async fn test_toggle_all_colors() {
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
                <Toggle color=color />
            };
            let expected = color.prefix("toggle");
            assert!(
                html.contains(&expected),
                "Missing color class {} for {:?}",
                expected,
                color
            );
        }
    }

    #[tokio::test]
    async fn test_toggle_all_sizes() {
        for size in [Size::Xs, Size::Sm, Size::Md, Size::Lg, Size::Xl] {
            let html = rsx! {
                <Toggle size=size />
            };
            let expected = size.prefix("toggle");
            assert!(
                html.contains(&expected),
                "Missing size class {} for {:?}",
                expected,
                size
            );
        }
    }

    #[tokio::test]
    async fn test_toggle_checked() {
        let html = rsx! {
            <Toggle checked=true />
        };
        assert!(html.contains(" checked"));
    }

    #[tokio::test]
    async fn test_toggle_disabled() {
        let html = rsx! {
            <Toggle disabled=true />
        };
        assert!(html.contains(" disabled"));
    }

    #[tokio::test]
    async fn test_toggle_with_name_and_value() {
        let html = rsx! {
            <Toggle name="notifications" value="on" />
        };
        assert!(html.contains(r#"name="notifications""#));
        assert!(html.contains(r#"value="on""#));
    }

    #[tokio::test]
    async fn test_toggle_custom_class() {
        let html = rsx! {
            <Toggle class="my-toggle" />
        };
        assert!(html.contains("my-toggle"));
    }

    #[tokio::test]
    async fn test_toggle_no_empty_optional_attrs() {
        let html = rsx! {
            <Toggle />
        };
        assert!(!html.contains("name="), "name should be omitted when None");
        assert!(
            !html.contains("value="),
            "value should be omitted when None"
        );
    }
}
