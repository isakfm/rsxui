//! Textarea Component
//!
//! A textarea component with DaisyUI styling.
//!
//! # DaisyUI Classes
//! - Base: `textarea`
//! - Colors: `textarea-primary`, `textarea-secondary`, etc.
//! - Sizes: `textarea-xs`, `textarea-sm`, `textarea-md`, `textarea-lg`, `textarea-xl`
//! - Styles: `textarea-ghost`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{Textarea, Color, Size};
//! use rsx::rsx;
//!
//! let html = rsx! {
//!     <Textarea placeholder="Enter message..." color={Color::Primary} size={Size::Lg} />
//! };
//! ```

use enum_stringify::EnumStringify;
use rsx::attrs::RenderAttrs;
use rsx::{classes, rsx, ui};

use super::{attr_if, Color, Size};

#[derive(Debug, Clone, PartialEq, Default, EnumStringify)]
#[enum_stringify(case = "kebab", prefix = "textarea-")]
pub enum TextareaStyle {
    #[default]
    #[enum_stringify(rename = "")]
    Default,
    Ghost,
}

#[ui]
pub fn Textarea(
    #[builder(into)] placeholder: Option<String>,
    #[builder(into)] value: Option<String>,
    #[builder(into)] name: Option<String>,
    size: Size,
    color: Color,
    #[builder(default)] style: TextareaStyle,
    disabled: bool,
    required: bool,
    readonly: bool,
    #[builder(default)] class: String,
) -> String {
    rsx! {
        <textarea
            class={classes!(
                "textarea",
                props.size.prefix("textarea"),
                props.color.prefix("textarea"),
                props.style,
                props.class,
            )}
            {attr_if("placeholder", &props.placeholder)}
            {attr_if("name", &props.name)}
            {props.render_attrs()}>{props.value.unwrap_or_default()}</textarea>
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
    async fn test_textarea_basic() {
        let html = rsx! {
            <Textarea />
        };
        println!("HTML: {}", html);
        assert!(html.contains("<textarea"));
        assert!(html.contains("class=\"textarea\""));
        assert!(html.contains("</textarea>"));
    }

    #[tokio::test]
    async fn test_textarea_sizes() {
        for size in [Size::Xs, Size::Sm, Size::Md, Size::Lg, Size::Xl] {
            let html = rsx! {
                <Textarea size=size />
            };
            let expected = size.prefix("textarea");
            assert!(
                html.contains(&expected),
                "Missing size class {} for {:?}",
                expected,
                size
            );
        }
    }

    #[tokio::test]
    async fn test_textarea_colors() {
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
                <Textarea color=color />
            };
            let expected = color.prefix("textarea");
            assert!(
                html.contains(&expected),
                "Missing color class {} for {:?}",
                expected,
                color
            );
        }
    }

    #[tokio::test]
    async fn test_textarea_ghost_style() {
        let html = rsx! {
            <Textarea style=TextareaStyle::Ghost />
        };
        assert!(html.contains("textarea-ghost"));
    }

    #[tokio::test]
    async fn test_textarea_custom_class() {
        let html = rsx! {
            <Textarea class="my-textarea" />
        };
        assert!(html.contains("my-textarea"));
    }

    #[tokio::test]
    async fn test_textarea_placeholder() {
        let html = rsx! {
            <Textarea placeholder="Enter text..." />
        };
        assert!(html.contains(r#"placeholder="Enter text...""#));
    }

    #[tokio::test]
    async fn test_textarea_value() {
        let html = rsx! {
            <Textarea value="Hello world" />
        };
        assert!(html.contains(">Hello world</textarea>"));
    }

    #[tokio::test]
    async fn test_textarea_name() {
        let html = rsx! {
            <Textarea name="description" />
        };
        assert!(html.contains(r#"name="description""#));
    }

    #[tokio::test]
    async fn test_textarea_boolean_props() {
        let html = rsx! {
            <Textarea disabled=true required=true readonly=true />
        };
        assert!(html.contains("disabled"));
        assert!(html.contains("required"));
        assert!(html.contains("readonly"));
    }

    #[tokio::test]
    async fn test_textarea_no_empty_optional_attrs() {
        let html = rsx! {
            <Textarea />
        };
        assert!(
            !html.contains("placeholder="),
            "placeholder should be omitted when None"
        );
        assert!(!html.contains("name="), "name should be omitted when None");
    }

    #[tokio::test]
    async fn test_textarea_with_html_attrs() {
        let html = rsx! {
            <Textarea id="my-textarea" title="Enter details" />
        };
        assert!(html.contains(r#"id="my-textarea""#));
        assert!(html.contains(r#"title="Enter details""#));
    }
}
