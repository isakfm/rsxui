//! Radio Component
//!
//! A radio input component with DaisyUI styling.
//!
//! # DaisyUI Classes
//! - Base: `radio`
//! - Colors: `radio-primary`, `radio-secondary`, etc.
//! - Sizes: `radio-xs`, `radio-sm`, `radio-md`, `radio-lg`, `radio-xl`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{Radio, Color, Size};
//! use rsx::rsx;
//!
//! let html = rsx! {
//!     <Radio name="group" color={Color::Primary} size={Size::Lg} checked=true />
//! };
//! ```

use rsx::{classes, component, rsx};

use super::{attr_if, show_if, Color, Size};

// ============================================
// Radio
// ============================================

#[component]
pub fn Radio(
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
            type="radio"
            class={classes!(
                "radio",
                color.prefix("radio"),
                size.prefix("radio"),
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
    async fn test_radio_basic() {
        let html = rsx! {
            <Radio />
        };
        assert!(html.contains("<input"));
        assert!(html.contains(r#"type="radio""#));
        assert!(html.contains("class=\"radio\""));
    }

    #[tokio::test]
    async fn test_radio_with_color() {
        let html = rsx! {
            <Radio color=Color::Primary />
        };
        assert!(html.contains("radio-primary"));
    }

    #[tokio::test]
    async fn test_radio_all_colors() {
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
                <Radio color=color />
            };
            let expected = color.prefix("radio");
            assert!(
                html.contains(&expected),
                "Missing color class {} for {:?}",
                expected,
                color
            );
        }
    }

    #[tokio::test]
    async fn test_radio_all_sizes() {
        for size in [Size::Xs, Size::Sm, Size::Md, Size::Lg, Size::Xl] {
            let html = rsx! {
                <Radio size=size />
            };
            let expected = size.prefix("radio");
            assert!(
                html.contains(&expected),
                "Missing size class {} for {:?}",
                expected,
                size
            );
        }
    }

    #[tokio::test]
    async fn test_radio_checked() {
        let html = rsx! {
            <Radio checked=true />
        };
        assert!(html.contains(" checked"));
    }

    #[tokio::test]
    async fn test_radio_disabled() {
        let html = rsx! {
            <Radio disabled=true />
        };
        assert!(html.contains(" disabled"));
    }

    #[tokio::test]
    async fn test_radio_with_name_and_value() {
        let html = rsx! {
            <Radio name="choice" value="option1" />
        };
        assert!(html.contains(r#"name="choice""#));
        assert!(html.contains(r#"value="option1""#));
    }

    #[tokio::test]
    async fn test_radio_custom_class() {
        let html = rsx! {
            <Radio class="my-radio" />
        };
        assert!(html.contains("my-radio"));
    }

    #[tokio::test]
    async fn test_radio_no_empty_optional_attrs() {
        let html = rsx! {
            <Radio />
        };
        assert!(!html.contains("name="), "name should be omitted when None");
        assert!(
            !html.contains("value="),
            "value should be omitted when None"
        );
    }
}
