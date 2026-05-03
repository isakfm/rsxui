// Divider Component
// Based on DaisyUI Divider: https://daisyui.com/components/divider/

use crate::components::Color;
use rsx::attrs::RenderAttrs;
use rsx::{classes, rsx, ui};

// ============================================
// Divider - Content separator
// ============================================

#[ui]
pub fn Divider(
    #[builder(default)] text: String,
    #[builder(default)] color: Color,
    #[builder(default)] vertical: bool,
    #[builder(default)] start: bool,
    #[builder(default)] end: bool,
    #[builder(default)] class: String,
) -> String {
    rsx! {
        <div class={classes!(
            "divider",
            props.color.prefix("divider"),
            class_if(props.vertical, "divider-vertical"),
            class_if(props.start, "divider-start"),
            class_if(props.end, "divider-end"),
            props.class
        )} {props.render_attrs()}>
            {props.text}
        </div>
    }
}

use crate::components::class_if;

// ============================================
// Tests
// ============================================

#[cfg(test)]
mod tests {
    use super::*;
    use rsx::rsx;

    #[tokio::test]
    async fn test_divider_basic() {
        let html = rsx! {
            <Divider />
        };
        assert!(html.contains("class=\"divider\""));
    }

    #[tokio::test]
    async fn test_divider_with_text() {
        let html = rsx! {
            <Divider text="OR" />
        };
        assert!(html.contains("OR"));
    }

    #[tokio::test]
    async fn test_divider_color() {
        let html = rsx! {
            <Divider color=Color::Primary />
        };
        assert!(html.contains("divider-primary"));
    }

    #[tokio::test]
    async fn test_divider_vertical() {
        let html = rsx! {
            <Divider vertical=true />
        };
        assert!(html.contains("divider-vertical"));
    }

    #[tokio::test]
    async fn test_divider_start() {
        let html = rsx! {
            <Divider start=true />
        };
        assert!(html.contains("divider-start"));
    }

    #[tokio::test]
    async fn test_divider_end() {
        let html = rsx! {
            <Divider end=true />
        };
        assert!(html.contains("divider-end"));
    }
}
