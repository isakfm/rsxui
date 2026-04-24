//! Tab Component
//!
//! Tabs can be used to show a list of links in a tabbed format.
//!
//! # DaisyUI Classes
//! - Base: `tabs`
//! - Part: `tab`, `tab-content`
//! - Style: `tabs-box`, `tabs-border`, `tabs-lift`
//! - Modifier: `tab-active`, `tab-disabled`
//! - Placement: `tabs-top`, `tabs-bottom`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{Tabs, Tab, TabStyle, TabPlacement};
//! use rsx_macros::rsx;
//!
//! let html = rsx! {
//!     <Tabs style={TabStyle::Lift}>
//!         <Tab active=true>"Tab 1"</Tab>
//!         <Tab>"Tab 2"</Tab>
//!         <Tab disabled=true>"Tab 3"</Tab>
//!     </Tabs>
//! };
//! ```

use enum_stringify::EnumStringify;
use rsx_macros::{classes, component, rsx};

use super::{class_if, show_if};

// ============================================
// TabStyle - Tab container style
// ============================================

#[derive(Debug, Clone, PartialEq, Default, EnumStringify)]
#[enum_stringify(case = "kebab", prefix = "tabs-")]
pub enum TabStyle {
    #[default]
    #[enum_stringify(rename = "")]
    Default,
    Box,
    Border,
    Lift,
}

// ============================================
// TabPlacement - Tab content placement
// ============================================

#[derive(Debug, Clone, PartialEq, Default, EnumStringify)]
#[enum_stringify(case = "kebab", prefix = "tabs-")]
pub enum TabPlacement {
    #[default]
    #[enum_stringify(rename = "")]
    Default,
    Top,
    Bottom,
}

// ============================================
// Tab - Button-based tab
// ============================================

#[component]
pub fn Tab(
    #[builder(default)] active: bool,
    #[builder(default)] disabled: bool,
    #[builder(default)] class: String,
    children: String,
) -> String {
    rsx! {
        <button
            role="tab"
            class={classes!(
                "tab",
                class_if(active, "tab-active"),
                class_if(disabled, "tab-disabled"),
                class,
            )}>
            {children}
        </button>
    }
}

// ============================================
// TabRadio - Radio input-based tab
// ============================================

#[component]
pub fn TabRadio(
    name: String,
    aria_label: String,
    #[builder(default)] checked: bool,
    #[builder(default)] class: String,
) -> String {
    rsx! {
        <input
            type="radio"
            name={name}
            aria-label={aria_label}
            class={classes!("tab", class)}
            {show_if(checked, "checked")} />
    }
}

// ============================================
// TabContent - Tab content panel
// ============================================

#[component]
pub fn TabContent(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div role="tabpanel" class={classes!("tab-content", class)}>
            {children}
        </div>
    }
}

// ============================================
// Tabs - Tab container
// ============================================

#[component]
pub fn Tabs(
    #[builder(default)] style: TabStyle,
    #[builder(default)] placement: TabPlacement,
    #[builder(default)] class: String,
    children: String,
) -> String {
    rsx! {
        <div
            role="tablist"
            class={classes!("tabs", style.to_string(), placement.to_string(), class)}>
            {children}
        </div>
    }
}

// ============================================
// Tests
// ============================================

#[cfg(test)]
mod tests {
    use super::*;
    use rsx_macros::rsx;

    #[tokio::test]
    async fn test_tabs_basic() {
        let html = rsx! {
            <Tabs>
                <Tab>"Tab 1"</Tab>
                <Tab>"Tab 2"</Tab>
            </Tabs>
        };
        assert!(html.contains(r#"role="tablist""#));
        assert!(html.contains(r#"class="tabs""#));
        assert!(html.contains(r#"role="tab""#));
        assert!(html.contains("tab"));
        assert!(html.contains(">Tab 1</button>"));
        assert!(html.contains(">Tab 2</button>"));
    }

    #[tokio::test]
    async fn test_tab_styles() {
        for style in [TabStyle::Box, TabStyle::Border, TabStyle::Lift] {
            let html = rsx! {
                <Tabs style=style.clone()>
                    <Tab>"X"</Tab>
                </Tabs>
            };
            let expected = style.to_string();
            assert!(
                html.contains(&expected),
                "Missing style class {} for {:?}",
                expected,
                style
            );
        }
    }

    #[tokio::test]
    async fn test_tab_placements() {
        for placement in [TabPlacement::Top, TabPlacement::Bottom] {
            let html = rsx! {
                <Tabs placement=placement.clone()>
                    <Tab>"X"</Tab>
                </Tabs>
            };
            let expected = placement.to_string();
            assert!(
                html.contains(&expected),
                "Missing placement class {} for {:?}",
                expected,
                placement
            );
        }
    }

    #[tokio::test]
    async fn test_tab_active() {
        let html = rsx! {
            <Tab active=true>"Active"</Tab>
        };
        assert!(html.contains("tab-active"));
    }

    #[tokio::test]
    async fn test_tab_disabled() {
        let html = rsx! {
            <Tab disabled=true>"Disabled"</Tab>
        };
        assert!(html.contains("tab-disabled"));
    }

    #[tokio::test]
    async fn test_tab_radio() {
        let html = rsx! {
            <TabRadio name="my_tabs" aria_label="Tab 1" checked=true />
        };
        assert!(html.contains(r#"type="radio""#));
        assert!(html.contains(r#"name="my_tabs""#));
        assert!(html.contains(r#"aria-label="Tab 1""#));
        assert!(html.contains("checked"));
        assert!(html.contains("tab"));
    }

    #[tokio::test]
    async fn test_tab_content() {
        let html = rsx! {
            <TabContent>"Content"</TabContent>
        };
        assert!(html.contains(r#"role="tabpanel""#));
        assert!(html.contains("tab-content"));
        assert!(html.contains(">Content</div>"));
    }

    #[tokio::test]
    async fn test_tabs_custom_class() {
        let html = rsx! {
            <Tabs class="tabs-lg">
                <Tab class="tab-lg">"Large"</Tab>
            </Tabs>
        };
        assert!(html.contains("tabs tabs-lg"));
        assert!(html.contains("tab tab-lg"));
    }

    #[tokio::test]
    async fn test_tab_default_no_extra_classes() {
        let html = rsx! {
            <Tab>"Default"</Tab>
        };
        assert!(!html.contains("tab-active"));
        assert!(!html.contains("tab-disabled"));
    }
}
