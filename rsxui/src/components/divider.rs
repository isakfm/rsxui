// Divider Component
// Based on DaisyUI Divider: https://daisyui.com/components/divider/

use crate::components::Color;
use rsx::{classes, component, rsx};

// ============================================
// Divider - Content separator
// ============================================

#[component]
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
            color.prefix("divider"),
            class_if(vertical, "divider-vertical"),
            class_if(start, "divider-start"),
            class_if(end, "divider-end"),
            class
        )}>
            {text}
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
