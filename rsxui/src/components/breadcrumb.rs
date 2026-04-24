//! Breadcrumb Component
//!
//! Breadcrumbs helps users to navigate.
//!
//! # DaisyUI Classes
//! - Base: `breadcrumbs`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::Breadcrumb;
//! use rsx_macros::rsx;
//!
//! let html = rsx! {
//!     <Breadcrumb>
//!         <li><a>"Home"</a></li>
//!         <li><a>"Docs"</a></li>
//!         <li>"Page"</li>
//!     </Breadcrumb>
//! };
//! ```

use rsx_macros::{classes, component, rsx};

// No additional imports needed

// ============================================
// Breadcrumb - Navigation breadcrumb
// ============================================

#[component]
pub fn Breadcrumb(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div class={classes!("breadcrumbs", class)}>
            <ul>{children}</ul>
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
    async fn test_breadcrumb_basic() {
        let html = rsx! {
            <Breadcrumb>
                <li><a>"Home"</a></li>
                <li><a>"Docs"</a></li>
                <li>"Page"</li>
            </Breadcrumb>
        };
        assert!(html.contains(r#"class="breadcrumbs""#));
        assert!(html.contains("<ul>"));
        assert!(html.contains(">Home</a>"));
        assert!(html.contains(">Page</li>"));
    }

    #[tokio::test]
    async fn test_breadcrumb_custom_class() {
        let html = rsx! {
            <Breadcrumb class="text-sm">"X"</Breadcrumb>
        };
        assert!(html.contains("text-sm"));
    }
}
