//! File Input Component
//!
//! A file input component with DaisyUI styling.
//!
//! # DaisyUI Classes
//! - Base: `file-input`
//! - Colors: `file-input-primary`, `file-input-secondary`, etc.
//! - Sizes: `file-input-xs`, `file-input-sm`, `file-input-md`, `file-input-lg`, `file-input-xl`
//! - Styles: `file-input-ghost`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{FileInput, Color, Size};
//! use rsx::rsx;
//!
//! let html = rsx! {
//!     <FileInput color={Color::Primary} size={Size::Lg} />
//! };
//! ```

use enum_stringify::EnumStringify;
use rsx::attrs::RenderAttrs;
use rsx::{classes, rsx, ui};

use super::{attr_if, Color, Size};

#[derive(Debug, Clone, PartialEq, Default, EnumStringify)]
#[enum_stringify(case = "kebab", prefix = "file-input-")]
pub enum FileInputStyle {
    #[default]
    #[enum_stringify(rename = "")]
    Default,
    Ghost,
}

#[ui]
pub fn FileInput(
    #[builder(into)] name: Option<String>,
    #[builder(into)] accept: Option<String>,
    size: Size,
    color: Color,
    #[builder(default)] style: FileInputStyle,
    disabled: bool,
    required: bool,
    multiple: bool,
    #[builder(default)] class: String,
) -> String {
    rsx! {
        <input
            _type="file"
            class={classes!(
                "file-input",
                props.size.prefix("file-input"),
                props.color.prefix("file-input"),
                props.style,
                props.class,
            )}
            {attr_if("name", &props.name)}
            {attr_if("accept", &props.accept)}
            {props.render_attrs()} />
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
    async fn test_file_input_basic() {
        let html = rsx! {
            <FileInput />
        };
        println!("HTML: {}", html);
        assert!(html.contains("<input"));
        assert!(html.contains(r#"type="file""#));
        assert!(html.contains("class=\"file-input\""));
    }

    #[tokio::test]
    async fn test_file_input_sizes() {
        for size in [Size::Xs, Size::Sm, Size::Md, Size::Lg, Size::Xl] {
            let html = rsx! {
                <FileInput size=size />
            };
            let expected = size.prefix("file-input");
            assert!(
                html.contains(&expected),
                "Missing size class {} for {:?}",
                expected,
                size
            );
        }
    }

    #[tokio::test]
    async fn test_file_input_colors() {
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
                <FileInput color=color />
            };
            let expected = color.prefix("file-input");
            assert!(
                html.contains(&expected),
                "Missing color class {} for {:?}",
                expected,
                color
            );
        }
    }

    #[tokio::test]
    async fn test_file_input_ghost_style() {
        let html = rsx! {
            <FileInput style=FileInputStyle::Ghost />
        };
        assert!(html.contains("file-input-ghost"));
    }

    #[tokio::test]
    async fn test_file_input_custom_class() {
        let html = rsx! {
            <FileInput class="my-file-input" />
        };
        assert!(html.contains("my-file-input"));
    }

    #[tokio::test]
    async fn test_file_input_name_and_accept() {
        let html = rsx! {
            <FileInput name="avatar" accept="image/*" />
        };
        assert!(html.contains(r#"name="avatar""#));
        assert!(html.contains(r#"accept="image/*""#));
    }

    #[tokio::test]
    async fn test_file_input_boolean_props() {
        let html = rsx! {
            <FileInput disabled=true required=true multiple=true />
        };
        assert!(html.contains("disabled"));
        assert!(html.contains("required"));
        assert!(html.contains("multiple"));
    }

    #[tokio::test]
    async fn test_file_input_no_empty_optional_attrs() {
        let html = rsx! {
            <FileInput />
        };
        assert!(!html.contains("name="), "name should be omitted when None");
        assert!(
            !html.contains("accept="),
            "accept should be omitted when None"
        );
    }

    #[tokio::test]
    async fn test_file_input_with_html_attrs() {
        let html = rsx! {
            <FileInput id="my-file" title="Upload a file" />
        };
        assert!(html.contains(r#"id="my-file""#));
        assert!(html.contains(r#"title="Upload a file""#));
    }
}
