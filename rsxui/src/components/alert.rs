//! Alert Component
//!
//! An alert component with DaisyUI styling.
//!
//! # DaisyUI Classes
//! - Base: `alert`
//! - Colors: `alert-info`, `alert-success`, `alert-warning`, `alert-error`
//! - Soft: `alert-soft`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{Alert, Color};
//! use rsx_macros::rsx;
//!
//! let html = rsx! {
//!     <Alert color={Color::Info}>"This is an info alert"</Alert>
//! };
//! ```

use rsx::attrs::RenderAttrs;
use rsx_macros::{classes, rsx, ui};

use super::Color;

#[ui]
pub fn Alert(
    color: Color,
    soft: bool,
    #[builder(default)] class: String,
    children: String,
) -> String {
    rsx! {
        <div
            class={classes!(
                "alert",
                props.color.prefix("alert"),
                super::class_if(props.soft, "alert-soft"),
                props.class,
            )}
            role="alert"
            {props.render_attrs()}>{props.children}</div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rsx_macros::rsx;

    #[tokio::test]
    async fn test_alert_basic() {
        let html = rsx! {
            <Alert>"Hello"</Alert>
        };
        assert!(html.contains("<div"));
        assert!(html.contains("class=\"alert\""));
        assert!(html.contains("role=\"alert\""));
        assert!(html.contains(">Hello</div>"));
    }

    #[tokio::test]
    async fn test_alert_with_color() {
        let html = rsx! {
            <Alert color=Color::Info>"Info"</Alert>
        };
        assert!(html.contains("alert-info"));
    }

    #[tokio::test]
    async fn test_alert_soft() {
        let html = rsx! {
            <Alert color=Color::Success soft=true>"Success"</Alert>
        };
        assert!(html.contains("alert-soft"));
        assert!(html.contains("alert-success"));
    }
}
