//! Button Component
//!
//! A button component with DaisyUI styling.
//!
//! # DaisyUI Classes
//! - Base: `btn`
//! - Colors: `btn-primary`, `btn-secondary`, etc.
//! - Sizes: `btn-xs`, `btn-sm`, `btn-md`, `btn-lg`, `btn-xl`
//! - Styles: `btn-outline`, `btn-ghost`, `btn-link`, etc.
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{Button, ButtonProps, Color, Size};
//!
//! let html = rsx! {
//!     <Button label="Click me" color={Color::Primary} size={Size::Lg} />
//! };
//! ```

use super::{Color, Size};
use enum_stringify::EnumStringify;
use rsx::attrs::RenderAttrs;
use rsx::{classes, rsx, ui};

pub use super::Size as ButtonSize;

#[derive(Debug, Clone, PartialEq, Default, EnumStringify)]
#[enum_stringify(case = "kebab", prefix = "btn-")]
pub enum ButtonStyle {
    #[default]
    #[enum_stringify(rename = "")]
    Default,
    Outline,
    Dash,
    Soft,
    Ghost,
    Link,
}

#[derive(Debug, Clone, PartialEq, Default, EnumStringify)]
#[enum_stringify(case = "lower")]
pub enum ButtonType {
    #[default]
    Button,
    Submit,
    Reset,
}

#[derive(Debug, Clone, PartialEq, Default, EnumStringify)]
#[enum_stringify(case = "kebab", prefix = "btn-")]
pub enum ButtonModifier {
    #[default]
    #[enum_stringify(rename = "")]
    Default,
    Wide,
    Block,
    Square,
    Circle,
}

#[ui]
pub fn Button(
    label: String,
    color: Color,
    size: Size,
    #[builder(default)] style: ButtonStyle,
    #[builder(default)] modifier: ButtonModifier,
    disabled: bool,
    loading: bool,
    #[builder(default)] button_type: ButtonType,
    #[builder(default)] class: String,
) -> String {
    rsx! {
        <button
            _type={props.button_type.to_string()}
            class={classes!(
                "btn",
                props.color.prefix("btn"),
                props.size.prefix("btn"),
                props.style,
                props.modifier,
                props.class,
            )}{props.render_attrs()}>{props.label}</button>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rsx::rsx;

    #[tokio::test]
    async fn test_button_basic() {
        let html = rsx! {
            <Button label="Click me" />
        };
        assert!(html.contains("<button"));
        assert!(html.contains("class=\"btn\""));
        assert!(html.contains(">Click me</button>"));
        // Verify no trailing space before >
        assert!(
            !html.contains(" \">"),
            "Found trailing space before >: {}",
            html
        );
    }

    #[tokio::test]
    async fn test_button_with_rsx() {
        let html = rsx! {
            <Button label="Click me" />
        };
        assert!(html.contains(r#"<button type="button" class="btn""#));
        assert!(html.contains(">Click me</button>"));
    }

    #[tokio::test]
    async fn test_button_with_color() {
        let html = rsx! {
            <Button label="Primary" color=Color::Primary />
        };
        assert!(html.contains("btn-primary"));
    }

    #[tokio::test]
    async fn test_button_all_colors() {
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
                <Button label="Test" color=color />
            };
            let expected = color.prefix("btn");
            assert!(
                html.contains(&expected),
                "Missing color class {} for {:?}",
                expected,
                color
            );
        }
    }

    #[tokio::test]
    async fn test_button_sizes() {
        for size in [Size::Xs, Size::Sm, Size::Md, Size::Lg, Size::Xl] {
            let html = rsx! {
                <Button label="Test" size=size />
            };
            let expected = size.prefix("btn");
            assert!(
                html.contains(&expected),
                "Missing size class {} for {:?}",
                expected,
                size
            );
        }
    }

    #[tokio::test]
    async fn test_button_with_id() {
        let html = rsx! {
            <Button label="With ID" id="my-button" />
        };
        assert!(html.contains(r#"id="my-button""#));
    }

    #[tokio::test]
    async fn test_button_with_name() {
        let html = rsx! {
            <Button label="With Name" name="my-name" />
        };
        assert!(html.contains(r#"name="my-name""#));
    }

    #[tokio::test]
    async fn test_button_with_title() {
        let html = rsx! {
            <Button label="With Title" title="Click me!" />
        };
        assert!(html.contains(r#"title="Click me!""#));
    }

    #[tokio::test]
    async fn test_button_with_htmx_post() {
        let html = rsx! {
            <Button label="HTMX" hx_post="/api/submit" hx_swap="outerHTML" />
        };
        assert!(html.contains(r#"hx-post="/api/submit""#));
        assert!(html.contains(r#"hx-swap="outerHTML""#));
    }

    #[tokio::test]
    async fn test_button_with_multiple_attributes() {
        let html = rsx! {
            <Button
                label="Full Featured"
                color=Color::Primary
                size=Size::Lg
                id="featured-btn"
                name="featured-button"
                title="A featured button"
                hx_post="/api/click"
            />
        };
        println!("{}", html.clone());
        assert!(html.contains(r#"id="featured-btn""#));
        assert!(html.contains(r#"name="featured-button""#));
        assert!(html.contains(r#"title="A featured button""#));
        assert!(html.contains(r#"hx-post="/api/click""#));
        assert!(html.contains("btn-primary"));
        assert!(html.contains("btn-lg"));
    }

    #[tokio::test]
    async fn test_button_custom_class() {
        let html = rsx! {
            <Button label="Custom" class="custom-class" disabled=true />
        };
        println!("{}", html);
        assert!(html.contains("custom-class"));
    }

    #[tokio::test]
    async fn test_button_with_modifier() {
        // Default modifier should not add any extra class
        let html_default = rsx! {
            <Button label="Default" />
        };
        assert!(html_default.contains("class=\"btn\""));
        assert!(!html_default.contains("btn-wide"));

        for modifier in [
            ButtonModifier::Wide,
            ButtonModifier::Block,
            ButtonModifier::Square,
            ButtonModifier::Circle,
        ] {
            let html = rsx! {
                <Button label="Modified" modifier=modifier.clone() />
            };
            let expected = modifier.to_string();
            assert!(
                html.contains(&expected),
                "Missing modifier class {} for {:?}",
                expected,
                modifier
            );
        }
    }

    #[tokio::test]
    async fn test_button_disabled() {
        let html = rsx! {
            <Button label="Disabled" disabled=true />
        };
        assert!(html.contains("disabled"));
    }

    #[tokio::test]
    async fn test_button_with_style() {
        // Default style should not add any extra class
        let html_default = rsx! {
            <Button label="Default" />
        };
        assert!(html_default.contains("class=\"btn\""));
        assert!(!html_default.contains("btn-outline"));

        for style in [
            ButtonStyle::Outline,
            ButtonStyle::Dash,
            ButtonStyle::Soft,
            ButtonStyle::Ghost,
            ButtonStyle::Link,
        ] {
            let html = rsx! {
                <Button label="Styled" style=style.clone() />
            };
            let expected = style.to_string();
            println!("{}", html.clone());
            assert!(
                html.contains(&expected),
                "Missing style class {} for {:?}",
                expected,
                style
            );
        }
    }
}
