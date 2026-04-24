//! Range Component
//!
//! A range slider input component with DaisyUI styling.
//!
//! # DaisyUI Classes
//! - Base: `range`
//! - Colors: `range-primary`, `range-secondary`, etc.
//! - Sizes: `range-xs`, `range-sm`, `range-md`, `range-lg`, `range-xl`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{Range, Color, Size};
//! use rsx_macros::rsx;
//!
//! let html = rsx! {
//!     <Range min="0" max="100" value="50" color={Color::Primary} size={Size::Lg} />
//! };
//! ```

use rsx_macros::{classes, component, rsx};

use super::{attr_if, show_if, Color, Size};

// ============================================
// Range
// ============================================

#[component]
pub fn Range(
    #[builder(into)] name: Option<String>,
    #[builder(into)] min: Option<String>,
    #[builder(into)] max: Option<String>,
    #[builder(into)] value: Option<String>,
    #[builder(default)] color: Color,
    #[builder(default)] size: Size,
    #[builder(default)] disabled: bool,
    #[builder(default)] class: String,
) -> String {
    rsx! {
        <input
            type="range"
            class={classes!(
                "range",
                color.prefix("range"),
                size.prefix("range"),
                class,
            )}
            {show_if(disabled, "disabled")}
            {attr_if("name", &name)}
            {attr_if("min", &min)}
            {attr_if("max", &max)}
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
    async fn test_range_basic() {
        let html = rsx! {
            <Range />
        };
        assert!(html.contains("<input"));
        assert!(html.contains(r#"type="range""#));
        assert!(html.contains("class=\"range\""));
    }

    #[tokio::test]
    async fn test_range_with_color() {
        let html = rsx! {
            <Range color=Color::Primary />
        };
        assert!(html.contains("range-primary"));
    }

    #[tokio::test]
    async fn test_range_all_colors() {
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
                <Range color=color />
            };
            let expected = color.prefix("range");
            assert!(
                html.contains(&expected),
                "Missing color class {} for {:?}",
                expected,
                color
            );
        }
    }

    #[tokio::test]
    async fn test_range_all_sizes() {
        for size in [Size::Xs, Size::Sm, Size::Md, Size::Lg, Size::Xl] {
            let html = rsx! {
                <Range size=size />
            };
            let expected = size.prefix("range");
            assert!(
                html.contains(&expected),
                "Missing size class {} for {:?}",
                expected,
                size
            );
        }
    }

    #[tokio::test]
    async fn test_range_disabled() {
        let html = rsx! {
            <Range disabled=true />
        };
        assert!(html.contains(" disabled"));
    }

    #[tokio::test]
    async fn test_range_with_min_max_value() {
        let html = rsx! {
            <Range min="0" max="100" value="50" />
        };
        assert!(html.contains(r#"min="0""#));
        assert!(html.contains(r#"max="100""#));
        assert!(html.contains(r#"value="50""#));
    }

    #[tokio::test]
    async fn test_range_with_name() {
        let html = rsx! {
            <Range name="volume" />
        };
        assert!(html.contains(r#"name="volume""#));
    }

    #[tokio::test]
    async fn test_range_custom_class() {
        let html = rsx! {
            <Range class="my-range" />
        };
        assert!(html.contains("my-range"));
    }

    #[tokio::test]
    async fn test_range_no_empty_optional_attrs() {
        let html = rsx! {
            <Range />
        };
        assert!(!html.contains("name="), "name should be omitted when None");
        assert!(!html.contains("min="), "min should be omitted when None");
        assert!(!html.contains("max="), "max should be omitted when None");
        assert!(
            !html.contains("value="),
            "value should be omitted when None"
        );
    }
}
