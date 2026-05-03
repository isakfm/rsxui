// Collapse Component
// Based on DaisyUI Collapse: https://daisyui.com/components/collapse/

use enum_stringify::EnumStringify;
use rsx::attrs::RenderAttrs;
use rsx::{classes, rsx, ui};

// ============================================
// CollapseModifier - Collapse modifier
// ============================================

#[derive(Debug, Clone, PartialEq, Default, EnumStringify)]
#[enum_stringify(case = "kebab", prefix = "collapse-")]
pub enum CollapseModifier {
    #[default]
    #[enum_stringify(rename = "")]
    Default,
    Arrow,
    Plus,
    Open,
    Close,
}

// ============================================
// Collapse - Toggle content visibility
// ============================================

#[ui]
pub fn Collapse(
    #[builder(default)] modifier: CollapseModifier,
    #[builder(default)] open: bool,
    #[builder(default)] class: String,
    children: String,
) -> String {
    let open_class = if props.open { "collapse-open" } else { "" };
    rsx! {
        <div tabindex="0" class={classes!("collapse", props.modifier, open_class, props.class)} {props.render_attrs()}>
            {props.children}
        </div>
    }
}

// ============================================
// CollapseTitle - Collapse title
// ============================================

#[ui]
pub fn CollapseTitle(children: String) -> String {
    rsx! {
        <div class="collapse-title" {props.render_attrs()}>{props.children}</div>
    }
}

// ============================================
// CollapseContent - Collapse content
// ============================================

#[ui]
pub fn CollapseContent(children: String) -> String {
    rsx! {
        <div class="collapse-content" {props.render_attrs()}>{props.children}</div>
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
    async fn test_collapse_basic() {
        let html = rsx! {
            <Collapse>
                <CollapseTitle>"Title"</CollapseTitle>
                <CollapseContent>"Content"</CollapseContent>
            </Collapse>
        };
        assert!(html.contains("class=\"collapse\""));
        assert!(html.contains("collapse-title"));
        assert!(html.contains("collapse-content"));
        assert!(html.contains("Title"));
        assert!(html.contains("Content"));
    }

    #[tokio::test]
    async fn test_collapse_arrow() {
        let html = rsx! {
            <Collapse modifier=CollapseModifier::Arrow>
                <CollapseTitle>"Title"</CollapseTitle>
                <CollapseContent>"Content"</CollapseContent>
            </Collapse>
        };
        assert!(html.contains("collapse-arrow"));
    }

    #[tokio::test]
    async fn test_collapse_plus() {
        let html = rsx! {
            <Collapse modifier=CollapseModifier::Plus>
                <CollapseTitle>"Title"</CollapseTitle>
                <CollapseContent>"Content"</CollapseContent>
            </Collapse>
        };
        assert!(html.contains("collapse-plus"));
    }

    #[tokio::test]
    async fn test_collapse_open() {
        let html = rsx! {
            <Collapse open=true>
                <CollapseTitle>"Title"</CollapseTitle>
                <CollapseContent>"Content"</CollapseContent>
            </Collapse>
        };
        assert!(html.contains("collapse-open"));
    }
}
