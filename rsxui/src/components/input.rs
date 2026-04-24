//! Input Component
//!
//! A text input component with DaisyUI styling.
//!
//! # DaisyUI Classes
//! - Base: `input`
//! - Colors: `input-primary`, `input-secondary`, etc.
//! - Sizes: `input-xs`, `input-sm`, `input-md`, `input-lg`, `input-xl`
//! - Styles: `input-ghost`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{Input, Color, Size};
//! use rsx_macros::rsx;
//!
//! let html = rsx! {
//!     <Input placeholder="Enter name" color={Color::Primary} size={Size::Lg} />
//! };
//! ```

use enum_stringify::EnumStringify;
use rsx::attrs::RenderAttrs;
use rsx_macros::{classes, rsx, ui};

use super::{attr_if, Color, Size};

#[derive(Debug, Clone, PartialEq, Default, EnumStringify)]
#[enum_stringify(case = "kebab", prefix = "input-")]
pub enum InputStyle {
    #[default]
    #[enum_stringify(rename = "")]
    Default,
    Ghost,
}

#[ui]
pub fn Input(
    input_type: String,
    #[builder(into)] placeholder: Option<String>,
    #[builder(into)] value: Option<String>,
    size: Size,
    color: Color,
    #[builder(default)] style: InputStyle,
    disabled: bool,
    required: bool,
    readonly: bool,
    #[builder(default)] class: String,
) -> String {
    let input_type_str = if props.input_type.is_empty() {
        "text".to_string()
    } else {
        props.input_type.clone()
    };

    rsx! {
        <input
            _type={input_type_str}
            class={classes!(
                "input",
                props.size.prefix("input"),
                props.color.prefix("input"),
                props.style,
                props.class,
            )}
            {attr_if("placeholder", &props.placeholder)}
            {attr_if("value", &props.value)}
            {props.render_attrs()} />
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rsx_macros::rsx;

    #[tokio::test]
    async fn test_input_basic() {
        let html = rsx! {
            <Input />
        };
        println!("HTML: {}", html);
        assert!(html.contains("<input"));
        assert!(html.contains(r#"type="text""#));
        assert!(html.contains("class=\"input\""));
    }

    #[tokio::test]
    async fn test_input_types() {
        for input_type in ["text", "email", "password", "number", "tel", "url"] {
            let html = rsx! {
                <Input input_type=input_type.to_string() />
            };
            assert!(html.contains(&format!(r#"type="{}""#, input_type)));
        }
    }

    #[tokio::test]
    async fn test_input_sizes() {
        for size in [Size::Xs, Size::Sm, Size::Md, Size::Lg, Size::Xl] {
            let html = rsx! {
                <Input size=size />
            };
            let expected = size.prefix("input");
            assert!(
                html.contains(&expected),
                "Missing size class {} for {:?}",
                expected,
                size
            );
        }
    }

    #[tokio::test]
    async fn test_input_boolean_props() {
        let html = rsx! {
            <Input disabled=true required=true readonly=true />
        };
        assert!(html.contains("disabled"));
        assert!(html.contains("required"));
        assert!(html.contains("readonly"));
    }

    #[tokio::test]
    async fn test_input_placeholder() {
        let html = rsx! {
            <Input placeholder="Enter text..." />
        };
        assert!(html.contains(r#"placeholder="Enter text...""#));
    }

    #[tokio::test]
    async fn test_input_no_empty_optional_attrs() {
        let html = rsx! {
            <Input />
        };
        // None values should not render empty attributes
        assert!(
            !html.contains("placeholder"),
            "placeholder should be omitted when None"
        );
        assert!(!html.contains("value"), "value should be omitted when None");
        assert!(!html.contains("name"), "name should be omitted when None");
    }

    #[tokio::test]
    async fn test_input_with_name_and_value() {
        let html = rsx! {
            <Input name="username" value="john" />
        };
        println!("{}", html);
        assert!(html.contains(r#"name="username""#));
        assert!(html.contains(r#"value="john""#));
    }

    #[tokio::test]
    async fn test_input_with_color() {
        let html = rsx! {
            <Input color=Color::Primary />
        };
        assert!(html.contains("input-primary"));
    }

    #[tokio::test]
    async fn test_input_with_ghost_style() {
        let html = rsx! {
            <Input style=InputStyle::Ghost />
        };
        assert!(html.contains("input-ghost"));
    }

    #[tokio::test]
    async fn test_input_with_custom_class() {
        let html = rsx! {
            <Input class="my-input" />
        };
        assert!(html.contains("my-input"));
    }

    #[tokio::test]
    async fn test_input_with_html_attrs() {
        let html = rsx! {
            <Input id="my-input" title="Enter text" />
        };
        assert!(html.contains(r#"id="my-input""#));
        assert!(html.contains(r#"title="Enter text""#));
    }
}
