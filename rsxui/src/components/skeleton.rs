//! Skeleton Component
//!
//! Skeleton is a component that can be used to show a loading state of a component.
//!
//! # DaisyUI Classes
//! - Base: `skeleton`
//! - Modifier: `skeleton-text`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::Skeleton;
//! use rsx_macros::rsx;
//!
//! let html = rsx! {
//!     <Skeleton class="w-32 h-32" />
//! };
//! ```

use rsx_macros::{classes, component, rsx};

use super::class_if;

// ============================================
// Skeleton - Loading placeholder
// ============================================

#[component]
pub fn Skeleton(
    #[builder(default)] text: bool,
    #[builder(default)] class: String,
    children: String,
) -> String {
    rsx! {
        <div class={classes!("skeleton", class_if(text, "skeleton-text"), class)}>
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
    async fn test_skeleton_basic() {
        let html = rsx! {
            <Skeleton class="w-32 h-32" />
        };
        assert!(html.contains(r#"class="skeleton w-32 h-32""#));
    }

    #[tokio::test]
    async fn test_skeleton_text() {
        let html = rsx! {
            <Skeleton text=true>"Loading..."</Skeleton>
        };
        assert!(html.contains("skeleton-text"));
        assert!(html.contains(">Loading...</div>"));
    }

    #[tokio::test]
    async fn test_skeleton_children() {
        let html = rsx! {
            <Skeleton class="h-4 w-20" />
        };
        assert!(html.contains("h-4"));
        assert!(html.contains("w-20"));
    }
}
