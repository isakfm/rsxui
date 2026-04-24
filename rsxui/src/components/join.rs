//! Join Component
//!
//! A container for grouping multiple items. Join applies border radius to the first and last item.
//!
//! # DaisyUI Classes
//! - Base: `join`
//! - Direction: `join-vertical`, `join-horizontal`
//! - Part: `join-item`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{Join, JoinItem};
//! use rsx_macros::rsx;
//!
//! let html = rsx! {
//!     <Join>
//!         <button class="btn join-item">"Button 1"</button>
//!         <button class="btn join-item">"Button 2"</button>
//!     </Join>
//! };
//! ```

use rsx_macros::{classes, component, rsx};

use super::class_if;

// ============================================
// JoinItem - Marks an element as part of a join
// ============================================

#[component]
pub fn JoinItem(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div class={classes!("join-item", class)}>
            {children}
        </div>
    }
}

// ============================================
// Join - Container for grouped items
// ============================================

#[component]
pub fn Join(
    #[builder(default)] vertical: bool,
    #[builder(default)] class: String,
    children: String,
) -> String {
    rsx! {
        <div class={classes!("join", class_if(vertical, "join-vertical"), class)}>
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
    async fn test_join_basic() {
        let html = rsx! {
            <Join>
                <button class="btn join-item">"1"</button>
                <button class="btn join-item">"2"</button>
            </Join>
        };
        assert!(html.contains(r#"class="join""#));
        assert!(html.contains(">1</button>"));
        assert!(html.contains(">2</button>"));
    }

    #[tokio::test]
    async fn test_join_vertical() {
        let html = rsx! {
            <Join vertical=true>
                <button class="btn join-item">"1"</button>
            </Join>
        };
        assert!(html.contains("join-vertical"));
    }

    #[tokio::test]
    async fn test_join_custom_class() {
        let html = rsx! {
            <Join class="my-join">
                <span>"A"</span>
            </Join>
        };
        assert!(html.contains("my-join"));
    }

    #[tokio::test]
    async fn test_join_item() {
        let html = rsx! {
            <JoinItem>
                <button class="btn">"X"</button>
            </JoinItem>
        };
        assert!(html.contains(r#"class="join-item""#));
        assert!(html.contains(">X</button>"));
    }
}
