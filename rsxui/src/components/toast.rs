//! Toast Component
//!
//! Toast is a wrapper to stack elements, positioned on the corner of page.
//!
//! # DaisyUI Classes
//! - Base: `toast`
//! - Placement: `toast-start`, `toast-center`, `toast-end`, `toast-top`, `toast-middle`, `toast-bottom`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{Toast, ToastHorizontal, ToastVertical};
//! use rsx_macros::rsx;
//!
//! let html = rsx! {
//!     <Toast horizontal={ToastHorizontal::End} vertical={ToastVertical::Top}>
//!         <div class="alert alert-info">"New message received"</div>
//!     </Toast>
//! };
//! ```

use enum_stringify::EnumStringify;
use rsx_macros::{classes, component, rsx};

// ============================================
// ToastHorizontal - Horizontal placement
// ============================================

#[derive(Debug, Clone, PartialEq, Default, EnumStringify)]
#[enum_stringify(case = "kebab", prefix = "toast-")]
pub enum ToastHorizontal {
    #[default]
    #[enum_stringify(rename = "")]
    Default,
    Start,
    Center,
    End,
}

// ============================================
// ToastVertical - Vertical placement
// ============================================

#[derive(Debug, Clone, PartialEq, Default, EnumStringify)]
#[enum_stringify(case = "kebab", prefix = "toast-")]
pub enum ToastVertical {
    #[default]
    #[enum_stringify(rename = "")]
    Default,
    Top,
    Middle,
    Bottom,
}

// ============================================
// Toast - Toast notification container
// ============================================

#[component]
pub fn Toast(
    #[builder(default)] horizontal: ToastHorizontal,
    #[builder(default)] vertical: ToastVertical,
    #[builder(default)] class: String,
    children: String,
) -> String {
    rsx! {
        <div class={classes!("toast", horizontal, vertical, class)}>
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
    async fn test_toast_basic() {
        let html = rsx! {
            <Toast>
                <div>"Message"</div>
            </Toast>
        };
        assert!(html.contains(r#"class="toast""#));
        assert!(html.contains(">Message</div>"));
    }

    #[tokio::test]
    async fn test_toast_horizontal_placements() {
        for placement in [
            ToastHorizontal::Start,
            ToastHorizontal::Center,
            ToastHorizontal::End,
        ] {
            let html = rsx! {
                <Toast horizontal=placement.clone()>"Test"</Toast>
            };
            let expected = placement.to_string();
            assert!(
                html.contains(&expected),
                "Missing horizontal class {} for {:?}",
                expected,
                placement
            );
        }
    }

    #[tokio::test]
    async fn test_toast_vertical_placements() {
        for placement in [
            ToastVertical::Top,
            ToastVertical::Middle,
            ToastVertical::Bottom,
        ] {
            let html = rsx! {
                <Toast vertical=placement.clone()>"Test"</Toast>
            };
            let expected = placement.to_string();
            assert!(
                html.contains(&expected),
                "Missing vertical class {} for {:?}",
                expected,
                placement
            );
        }
    }

    #[tokio::test]
    async fn test_toast_combined_placement() {
        let html = rsx! {
            <Toast horizontal=ToastHorizontal::End vertical=ToastVertical::Top>
                "Alert"
            </Toast>
        };
        assert!(html.contains("toast-end"));
        assert!(html.contains("toast-top"));
    }

    #[tokio::test]
    async fn test_toast_custom_class() {
        let html = rsx! {
            <Toast class="z-50">"Alert"</Toast>
        };
        assert!(html.contains("toast z-50"));
    }

    #[tokio::test]
    async fn test_toast_default_no_extra_classes() {
        let html = rsx! {
            <Toast>"Default"</Toast>
        };
        assert!(!html.contains("toast-start"));
        assert!(!html.contains("toast-top"));
    }
}
