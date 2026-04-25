//! FAB (Floating Action Button) Component
//!
//! FAB stays in the bottom corner of screen and shows additional
//! buttons (Speed Dial) when clicked or focused.
//!
//! # DaisyUI Classes
//! - Base: `fab`
//! - Part: `fab-close`, `fab-main-action`
//! - Modifier: `fab-flower`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{Fab, FabModifier};
//! use rsx_macros::rsx;
//!
//! let html = rsx! {
//!     <Fab modifier={FabModifier::Flower}>
//!         <div tabindex="0" role="button" class="btn btn-circle">"F"</div>
//!         <button class="btn btn-circle">"A"</button>
//!     </Fab>
//! };
//! ```

use enum_stringify::EnumStringify;
use rsx_macros::{classes, component, rsx};

// ============================================
// FabModifier - FAB layout modifier
// ============================================

#[derive(Debug, Clone, PartialEq, Default, EnumStringify)]
#[enum_stringify(case = "kebab", prefix = "fab-")]
pub enum FabModifier {
    #[default]
    #[enum_stringify(rename = "")]
    Default,
    Flower,
}

// ============================================
// FabClose - Close button wrapper for FAB
// ============================================

#[component]
pub fn FabClose(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div class={classes!("fab-close", class)}>{children}</div>
    }
}

// ============================================
// FabMainAction - Main action button wrapper for FAB
// ============================================

#[component]
pub fn FabMainAction(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div class={classes!("fab-main-action", class)}>{children}</div>
    }
}

// ============================================
// Fab - Floating Action Button container
// ============================================

#[component]
pub fn Fab(
    #[builder(default)] modifier: FabModifier,
    #[builder(default)] class: String,
    children: String,
) -> String {
    rsx! {
        <div class={classes!("fab", modifier, class)}>
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
    async fn test_fab_basic() {
        let html = rsx! {
            <Fab>
                <div tabindex="0" role="button" class="btn btn-circle">"F"</div>
            </Fab>
        };
        assert!(html.contains(r#"class="fab""#));
    }

    #[tokio::test]
    async fn test_fab_flower() {
        let html = rsx! {
            <Fab modifier=FabModifier::Flower>
                <div tabindex="0" role="button">"F"</div>
                <button>"A"</button>
            </Fab>
        };
        assert!(html.contains("fab fab-flower"));
    }

    #[tokio::test]
    async fn test_fab_close() {
        let html = rsx! {
            <FabClose>"Close" <span class="btn btn-error">"X"</span></FabClose>
        };
        assert!(html.contains(r#"class="fab-close""#));
        assert!(html.contains(">Close"));
        assert!(html.contains("</div>"));
    }

    #[tokio::test]
    async fn test_fab_main_action() {
        let html = rsx! {
            <FabMainAction>
                "Main" <button class="btn btn-secondary">"M"</button>
            </FabMainAction>
        };
        assert!(html.contains(r#"class="fab-main-action""#));
        assert!(html.contains(">Main"));
        assert!(html.contains("</div>"));
    }

    #[tokio::test]
    async fn test_fab_custom_class() {
        let html = rsx! {
            <Fab class="absolute z-1">
                <div tabindex="0" role="button">"F"</div>
            </Fab>
        };
        assert!(html.contains("fab absolute z-1"));
    }

    #[tokio::test]
    async fn test_fab_default_no_extra_classes() {
        let html = rsx! {
            <Fab>
                <div tabindex="0" role="button">"F"</div>
            </Fab>
        };
        assert!(!html.contains("fab-flower"));
    }

    #[tokio::test]
    async fn test_fab_composition() {
        let html = rsx! {
            <Fab modifier=FabModifier::Flower>
                <div tabindex="0" role="button" class="btn btn-circle btn-primary">"F"</div>
                <FabMainAction>
                    <button class="btn btn-circle">"M"</button>
                </FabMainAction>
                <FabClose>
                    <span class="btn btn-error">"X"</span>
                </FabClose>
                <button class="btn btn-circle">"A"</button>
            </Fab>
        };
        assert!(html.contains("fab fab-flower"));
        assert!(html.contains("fab-main-action"));
        assert!(html.contains("fab-close"));
    }
}
