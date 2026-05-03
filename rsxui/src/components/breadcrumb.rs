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
//! use rsx::rsx;
//!
//! let html = rsx! {
//!     <Breadcrumb>
//!         <li><a>"Home"</a></li>
//!         <li><a>"Docs"</a></li>
//!         <li>"Page"</li>
//!     </Breadcrumb>
//! };
//! ```

use rsx::attrs::RenderAttrs;
use rsx::{classes, rsx, ui};

// No additional imports needed

// ============================================
// Breadcrumb - Navigation breadcrumb
// ============================================

#[ui]
pub fn Breadcrumb(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div class={classes!("breadcrumbs", props.class)} {props.render_attrs()}>
            <ul>{props.children}</ul>
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
