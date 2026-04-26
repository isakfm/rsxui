// Collapse Component
// Based on DaisyUI Collapse: https://daisyui.com/components/collapse/

use enum_stringify::EnumStringify;
use rsx::{classes, component, rsx};

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

#[component]
pub fn Collapse(
    #[builder(default)] modifier: CollapseModifier,
    #[builder(default)] open: bool,
    #[builder(default)] class: String,
    children: String,
) -> String {
    let open_class = if open { "collapse-open" } else { "" };
    rsx! {
        <div tabindex="0" class={classes!("collapse", modifier, open_class, class)}>
            {children}
        </div>
    }
}

// ============================================
// CollapseTitle - Collapse title
// ============================================

#[component]
pub fn CollapseTitle(children: String) -> String {
    rsx! {
        <div class="collapse-title">{children}</div>
    }
}

// ============================================
// CollapseContent - Collapse content
// ============================================

#[component]
pub fn CollapseContent(children: String) -> String {
    rsx! {
        <div class="collapse-content">{children}</div>
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
