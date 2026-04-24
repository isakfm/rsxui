//! Pagination Component
//!
//! Pagination is a group of buttons for page navigation.
//!
//! # DaisyUI Classes
//! - Base: `join`
//! - Part: `join-item`, `btn`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{Pagination, PaginationItem};
//! use rsx_macros::rsx;
//!
//! let html = rsx! {
//!     <Pagination>
//!         <PaginationItem>"1"</PaginationItem>
//!         <PaginationItem active={true}>"2"</PaginationItem>
//!         <PaginationItem>"3"</PaginationItem>
//!     </Pagination>
//! };
//! ```

use crate::components::class_if;
use rsx_macros::{classes, component, rsx};

// ============================================
// PaginationItem - Individual page button
// ============================================

#[component]
pub fn PaginationItem(
    #[builder(default)] class: String,
    #[builder(default)] active: bool,
    #[builder(default)] disabled: bool,
    children: String,
) -> String {
    rsx! {
        <button
            class={classes!(
                "join-item",
                "btn",
                class_if(active, "btn-active"),
                class_if(disabled, "btn-disabled"),
                class,
            )}>
            {children}
        </button>
    }
}

// ============================================
// Pagination - Page navigation container
// ============================================

#[component]
pub fn Pagination(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div class={classes!("join", class)}>{children}</div>
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
    async fn test_pagination_basic() {
        let html = rsx! {
            <Pagination>
                <PaginationItem>"1"</PaginationItem>
                <PaginationItem>"2"</PaginationItem>
            </Pagination>
        };
        assert!(html.contains(r#"class="join""#));
        assert!(html.contains("join-item"));
        assert!(html.contains("btn"));
        assert!(html.contains(">1</button>"));
        assert!(html.contains(">2</button>"));
    }

    #[tokio::test]
    async fn test_pagination_item_active() {
        let html = rsx! {
            <PaginationItem active=true>"1"</PaginationItem>
        };
        assert!(html.contains("btn-active"));
    }

    #[tokio::test]
    async fn test_pagination_item_disabled() {
        let html = rsx! {
            <PaginationItem disabled=true>"1"</PaginationItem>
        };
        assert!(html.contains("btn-disabled"));
    }

    #[tokio::test]
    async fn test_pagination_custom_class() {
        let html = rsx! {
            <Pagination class="my-pagination">
                <PaginationItem class="w-12">"1"</PaginationItem>
            </Pagination>
        };
        assert!(html.contains("join my-pagination"));
        assert!(html.contains("w-12"));
    }

    #[tokio::test]
    async fn test_pagination_item_not_active_by_default() {
        let html = rsx! {
            <PaginationItem>"1"</PaginationItem>
        };
        assert!(!html.contains("btn-active"));
        assert!(!html.contains("btn-disabled"));
    }
}
