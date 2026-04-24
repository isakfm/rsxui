//! Checkbox Component
//!
//! A checkbox input component with DaisyUI styling.
//!
//! # DaisyUI Classes
//! - Base: `checkbox`
//! - Colors: `checkbox-primary`, `checkbox-secondary`, etc.
//! - Sizes: `checkbox-xs`, `checkbox-sm`, `checkbox-md`, `checkbox-lg`, `checkbox-xl`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{Checkbox, Color, Size};
//! use rsx_macros::rsx;
//!
//! let html = rsx! {
//!     <Checkbox color={Color::Primary} size={Size::Lg} checked=true />
//! };
//! ```

use rsx_macros::{classes, component, rsx};

use super::{attr_if, show_if, Color, Size};

// ============================================
// Checkbox
// ============================================

#[component]
pub fn Checkbox(
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
                "checkbox",
                color.prefix("checkbox"),
                size.prefix("checkbox"),
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
    use rsx_macros::rsx;

    #[tokio::test]
    async fn test_checkbox_basic() {
        let html = rsx! {
            <Checkbox />
        };
        assert!(html.contains("<input"));
        assert!(html.contains(r#"type="checkbox""#));
        assert!(html.contains("class=\"checkbox\""));
    }

    #[tokio::test]
    async fn test_checkbox_with_color() {
        let html = rsx! {
            <Checkbox color=Color::Primary />
        };
        assert!(html.contains("checkbox-primary"));
    }

    #[tokio::test]
    async fn test_checkbox_all_colors() {
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
                <Checkbox color=color />
            };
            let expected = color.prefix("checkbox");
            assert!(
                html.contains(&expected),
                "Missing color class {} for {:?}",
                expected,
                color
            );
        }
    }

    #[tokio::test]
    async fn test_checkbox_all_sizes() {
        for size in [Size::Xs, Size::Sm, Size::Md, Size::Lg, Size::Xl] {
            let html = rsx! {
                <Checkbox size=size />
            };
            let expected = size.prefix("checkbox");
            assert!(
                html.contains(&expected),
                "Missing size class {} for {:?}",
                expected,
                size
            );
        }
    }

    #[tokio::test]
    async fn test_checkbox_checked() {
        let html = rsx! {
            <Checkbox checked=true />
        };
        assert!(html.contains(" checked"));
    }

    #[tokio::test]
    async fn test_checkbox_disabled() {
        let html = rsx! {
            <Checkbox disabled=true />
        };
        assert!(html.contains(" disabled"));
    }

    #[tokio::test]
    async fn test_checkbox_with_name_and_value() {
        let html = rsx! {
            <Checkbox name="agree" value="yes" />
        };
        assert!(html.contains(r#"name="agree""#));
        assert!(html.contains(r#"value="yes""#));
    }

    #[tokio::test]
    async fn test_checkbox_custom_class() {
        let html = rsx! {
            <Checkbox class="my-checkbox" />
        };
        assert!(html.contains("my-checkbox"));
    }

    #[tokio::test]
    async fn test_checkbox_no_empty_optional_attrs() {
        let html = rsx! {
            <Checkbox />
        };
        assert!(!html.contains("name="), "name should be omitted when None");
        assert!(
            !html.contains("value="),
            "value should be omitted when None"
        );
    }
}
