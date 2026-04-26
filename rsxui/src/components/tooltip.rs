//! Tooltip Component
//!
//! Tooltip can be used to show a message when hovering over an element.
//!
//! # DaisyUI Classes
//! - Base: `tooltip`
//! - Modifier: `tooltip-open`
//! - Placement: `tooltip-top`, `tooltip-bottom`, `tooltip-left`, `tooltip-right`
//! - Color: `tooltip-primary`, `tooltip-secondary`, `tooltip-accent`, `tooltip-info`, `tooltip-success`, `tooltip-warning`, `tooltip-error`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{Tooltip, TooltipPlacement, Color};
//! use rsx::rsx;
//!
//! let html = rsx! {
//!     <Tooltip data_tip="Hello!" placement={TooltipPlacement::Top} color={Color::Primary}>
//!         <button class="btn">"Hover me"</button>
//!     </Tooltip>
//! };
//! ```

use enum_stringify::EnumStringify;
use rsx::attrs::RenderAttrs;
use rsx::{classes, rsx, ui};

use super::{class_if, Color};

// ============================================
// TooltipPlacement - Tooltip position
// ============================================

#[derive(Debug, Clone, PartialEq, Default, EnumStringify)]
#[enum_stringify(case = "kebab", prefix = "tooltip-")]
pub enum TooltipPlacement {
    #[default]
    #[enum_stringify(rename = "")]
    Default,
    Top,
    Bottom,
    Left,
    Right,
}

// ============================================
// Tooltip - Hover tooltip wrapper
// ============================================

#[ui]
pub fn Tooltip(
    data_tip: String,
    #[builder(default)] placement: TooltipPlacement,
    #[builder(default)] color: Color,
    #[builder(default)] open: bool,
    #[builder(default)] class: String,
    children: String,
) -> String {
    rsx! {
        <div
            class={classes!(
                "tooltip",
                props.placement,
                props.color.prefix("tooltip"),
                class_if(props.open, "tooltip-open"),
                props.class,
            )}
            data-tip={props.data_tip.clone()}
            {props.render_attrs()}>
            {props.children}
        </div>
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
    async fn test_tooltip_basic() {
        let html = rsx! {
            <Tooltip data_tip="Hello!">
                <button>"Hover me"</button>
            </Tooltip>
        };
        assert!(html.contains(r#"class="tooltip""#));
        assert!(html.contains(r#"data-tip="Hello!""#));
        assert!(html.contains(">Hover me</button>"));
    }

    #[tokio::test]
    async fn test_tooltip_placements() {
        for placement in [
            TooltipPlacement::Top,
            TooltipPlacement::Bottom,
            TooltipPlacement::Left,
            TooltipPlacement::Right,
        ] {
            let html = rsx! {
                <Tooltip data_tip="Test" placement=placement.clone()>
                    <button>"X"</button>
                </Tooltip>
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
    async fn test_tooltip_colors() {
        for color in [
            Color::Primary,
            Color::Secondary,
            Color::Accent,
            Color::Info,
            Color::Success,
            Color::Warning,
            Color::Error,
        ] {
            let html = rsx! {
                <Tooltip data_tip="Test" color=color>
                    <button>"X"</button>
                </Tooltip>
            };
            let expected = color.prefix("tooltip");
            assert!(
                html.contains(&expected),
                "Missing color class {} for {:?}",
                expected,
                color
            );
        }
    }

    #[tokio::test]
    async fn test_tooltip_open() {
        let html = rsx! {
            <Tooltip data_tip="Open" open=true>
                <button>"X"</button>
            </Tooltip>
        };
        assert!(html.contains("tooltip-open"));
    }

    #[tokio::test]
    async fn test_tooltip_custom_class() {
        let html = rsx! {
            <Tooltip data_tip="Test" class="my-tooltip">
                <button>"X"</button>
            </Tooltip>
        };
        assert!(html.contains("my-tooltip"));
    }

    #[tokio::test]
    async fn test_tooltip_default_no_extra_classes() {
        let html = rsx! {
            <Tooltip data_tip="Default">
                <button>"X"</button>
            </Tooltip>
        };
        assert!(!html.contains("tooltip-top"));
        assert!(!html.contains("tooltip-primary"));
        assert!(!html.contains("tooltip-open"));
    }
}
