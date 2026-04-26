//! Select Component
//!
//! A select dropdown component with DaisyUI styling.
//!
//! # DaisyUI Classes
//! - Base: `select`
//! - Colors: `select-primary`, `select-secondary`, etc.
//! - Sizes: `select-xs`, `select-sm`, `select-md`, `select-lg`, `select-xl`
//! - Styles: `select-ghost`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{Select, Color, Size};
//! use rsx::rsx;
//!
//! let html = rsx! {
//!     <Select color={Color::Primary} size={Size::Lg}>
//!         <option>"Option 1"</option>
//!     </Select>
//! };
//! ```

use enum_stringify::EnumStringify;
use rsx::attrs::RenderAttrs;
use rsx::{classes, rsx, ui};

use super::{attr_if, Color, Size};

#[derive(Debug, Clone, PartialEq, Default, EnumStringify)]
#[enum_stringify(case = "kebab", prefix = "select-")]
pub enum SelectStyle {
    #[default]
    #[enum_stringify(rename = "")]
    Default,
    Ghost,
}

#[ui]
pub fn Select(
    #[builder(into)] name: Option<String>,
    size: Size,
    color: Color,
    #[builder(default)] style: SelectStyle,
    disabled: bool,
    required: bool,
    multiple: bool,
    #[builder(default)] class: String,
    children: String,
) -> String {
    rsx! {
        <select
            class={classes!(
                "select",
                props.size.prefix("select"),
                props.color.prefix("select"),
                props.style,
                props.class,
            )}
            {attr_if("name", &props.name)}
            {props.render_attrs()}>{props.children}</select>
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
    async fn test_select_basic() {
        let html = rsx! {
            <Select>""</Select>
        };
        println!("HTML: {}", html);
        assert!(html.contains("<select"));
        assert!(html.contains("class=\"select\""));
        assert!(html.contains("</select>"));
    }

    #[tokio::test]
    async fn test_select_sizes() {
        for size in [Size::Xs, Size::Sm, Size::Md, Size::Lg, Size::Xl] {
            let html = rsx! {
                <Select size=size>""</Select>
            };
            let expected = size.prefix("select");
            assert!(
                html.contains(&expected),
                "Missing size class {} for {:?}",
                expected,
                size
            );
        }
    }

    #[tokio::test]
    async fn test_select_colors() {
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
                <Select color=color>""</Select>
            };
            let expected = color.prefix("select");
            assert!(
                html.contains(&expected),
                "Missing color class {} for {:?}",
                expected,
                color
            );
        }
    }

    #[tokio::test]
    async fn test_select_ghost_style() {
        let html = rsx! {
            <Select style=SelectStyle::Ghost>""</Select>
        };
        assert!(html.contains("select-ghost"));
    }

    #[tokio::test]
    async fn test_select_custom_class() {
        let html = rsx! {
            <Select class="my-select">""</Select>
        };
        assert!(html.contains("my-select"));
    }

    #[tokio::test]
    async fn test_select_name() {
        let html = rsx! {
            <Select name="country">""</Select>
        };
        assert!(html.contains(r#"name="country""#));
    }

    #[tokio::test]
    async fn test_select_boolean_props() {
        let html = rsx! {
            <Select disabled=true required=true multiple=true>""</Select>
        };
        assert!(html.contains("disabled"));
        assert!(html.contains("required"));
        assert!(html.contains("multiple"));
    }

    #[tokio::test]
    async fn test_select_no_empty_optional_attrs() {
        let html = rsx! {
            <Select>""</Select>
        };
        assert!(!html.contains("name="), "name should be omitted when None");
    }

    #[tokio::test]
    async fn test_select_with_html_attrs() {
        let html = rsx! {
            <Select id="my-select" title="Choose an option">""</Select>
        };
        assert!(html.contains(r#"id="my-select""#));
        assert!(html.contains(r#"title="Choose an option""#));
    }

    #[tokio::test]
    async fn test_select_with_children() {
        let html = rsx! {
            <Select>
                <option value="1">"One"</option>
                <option value="2">"Two"</option>
            </Select>
        };
        assert!(html.contains("One"));
        assert!(html.contains("Two"));
        assert!(html.contains(r#"value="1""#));
        assert!(html.contains(r#"value="2""#));
    }
}
