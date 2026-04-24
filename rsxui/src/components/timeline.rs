//! Timeline Component
//!
//! Timeline component shows a list of events in chronological order.
//!
//! # DaisyUI Classes
//! - Base: `timeline`
//! - Parts: `timeline-start`, `timeline-middle`, `timeline-end`
//! - Modifier: `timeline-snap-icon`, `timeline-compact`
//! - Direction: `timeline-vertical`, `timeline-horizontal`
//! - Box style: apply `timeline-box` class to `TimelineStart` or `TimelineEnd`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{Timeline, TimelineItem, TimelineDirection};
//! use rsx_macros::rsx;
//!
//! let html = rsx! {
//!     <Timeline>
//!         <TimelineItem>
//!             <TimelineStart>"1984"</TimelineStart>
//!             <TimelineMiddle>"●"</TimelineMiddle>
//!             <TimelineEnd>"First Macintosh computer"</TimelineEnd>
//!         </TimelineItem>
//!     </Timeline>
//! };
//! ```

use enum_stringify::EnumStringify;
use rsx_macros::{classes, component, rsx};

// ============================================
// TimelineDirection - Timeline layout direction
// ============================================

#[derive(Debug, Clone, PartialEq, Default, EnumStringify)]
#[enum_stringify(case = "kebab", prefix = "timeline-")]
pub enum TimelineDirection {
    #[default]
    #[enum_stringify(rename = "")]
    Default,
    Vertical,
    Horizontal,
}

// ============================================
// TimelineModifier - Timeline modifiers
// ============================================

#[derive(Debug, Clone, PartialEq, Default, EnumStringify)]
#[enum_stringify(case = "kebab", prefix = "timeline-")]
pub enum TimelineModifier {
    #[default]
    #[enum_stringify(rename = "")]
    Default,
    SnapIcon,
    Compact,
}

// ============================================
// TimelineStart - Start content of a timeline item
// ============================================

#[component]
pub fn TimelineStart(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div class={classes!("timeline-start", class)}>{children}</div>
    }
}

// ============================================
// TimelineMiddle - Middle icon/content of a timeline item
// ============================================

#[component]
pub fn TimelineMiddle(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div class={classes!("timeline-middle", class)}>{children}</div>
    }
}

// ============================================
// TimelineEnd - End content of a timeline item
// ============================================

#[component]
pub fn TimelineEnd(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div class={classes!("timeline-end", class)}>{children}</div>
    }
}

// ============================================
// TimelineItem - Individual timeline entry
// ============================================

#[component]
pub fn TimelineItem(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <li class={classes!(class)}>{children}</li>
    }
}

// ============================================
// Timeline - Timeline container
// ============================================

#[component]
pub fn Timeline(
    #[builder(default)] direction: TimelineDirection,
    #[builder(default)] modifier: TimelineModifier,
    #[builder(default)] class: String,
    children: String,
) -> String {
    rsx! {
        <ul class={classes!("timeline", direction.to_string(), modifier.to_string(), class)}>
            {children}
        </ul>
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
    async fn test_timeline_basic() {
        let html = rsx! {
            <Timeline>
                <TimelineItem>
                    <TimelineStart>"1984"</TimelineStart>
                    <TimelineMiddle>"●"</TimelineMiddle>
                    <TimelineEnd>"First Mac"</TimelineEnd>
                </TimelineItem>
            </Timeline>
        };
        assert!(html.contains(r#"class="timeline""#));
        assert!(html.contains("timeline-start"));
        assert!(html.contains("timeline-middle"));
        assert!(html.contains("timeline-end"));
        assert!(html.contains(">1984</div>"));
        assert!(html.contains(">First Mac</div>"));
    }

    #[tokio::test]
    async fn test_timeline_directions() {
        for direction in [TimelineDirection::Vertical, TimelineDirection::Horizontal] {
            let html = rsx! {
                <Timeline direction=direction.clone()>
                    <TimelineItem>"X"</TimelineItem>
                </Timeline>
            };
            let expected = direction.to_string();
            assert!(
                html.contains(&expected),
                "Missing direction class {} for {:?}",
                expected,
                direction
            );
        }
    }

    #[tokio::test]
    async fn test_timeline_modifiers() {
        for modifier in [TimelineModifier::SnapIcon, TimelineModifier::Compact] {
            let html = rsx! {
                <Timeline modifier=modifier.clone()>
                    <TimelineItem>"X"</TimelineItem>
                </Timeline>
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
    async fn test_timeline_custom_class() {
        let html = rsx! {
            <Timeline class="timeline-snap-icon">
                <TimelineItem class="mb-4">
                    <TimelineStart class="font-bold">"A"</TimelineStart>
                </TimelineItem>
            </Timeline>
        };
        assert!(html.contains("timeline timeline-snap-icon"));
        assert!(html.contains("timeline-start font-bold"));
    }

    #[tokio::test]
    async fn test_timeline_box_on_child() {
        let html = rsx! {
            <Timeline>
                <TimelineItem>
                    <TimelineStart class="timeline-box">"1984"</TimelineStart>
                    <TimelineMiddle>"●"</TimelineMiddle>
                    <TimelineEnd class="timeline-box">"First Mac"</TimelineEnd>
                </TimelineItem>
            </Timeline>
        };
        assert!(html.contains("timeline-start timeline-box"));
        assert!(html.contains("timeline-end timeline-box"));
    }

    #[tokio::test]
    async fn test_timeline_default_no_extra_classes() {
        let html = rsx! {
            <Timeline>
                <TimelineItem>"X"</TimelineItem>
            </Timeline>
        };
        assert!(!html.contains("timeline-vertical"));
        assert!(!html.contains("timeline-snap-icon"));
    }
}
