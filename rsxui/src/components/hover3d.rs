//! Hover 3D Component
//!
//! Hover 3D is a wrapper component that adds a 3D hover effect to its content.
//!
//! # DaisyUI Classes
//! - Base: `hover-3d`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::Hover3d;
//! use rsx_macros::rsx;
//!
//! let html = rsx! {
//!     <Hover3d>
//!         <img src="card.jpg" />
//!     </Hover3d>
//! };
//! ```

use rsx_macros::{classes, component, rsx};

// ============================================
// Hover3d - 3D hover effect wrapper
// ============================================

#[component]
pub fn Hover3d(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div class={classes!("hover-3d", class)}>
            {children}
            <div />
            <div />
            <div />
            <div />
            <div />
            <div />
            <div />
            <div />
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
    async fn test_hover3d_basic() {
        let html = rsx! {
            <Hover3d>
                <img src="photo.jpg" />
            </Hover3d>
        };
        assert!(html.contains(r#"class="hover-3d""#));
        assert!(html.contains("<img"));
        // Must have 8 empty divs after content (rendered as <div></div>)
        let empty_div_count = html.matches("<div></div>").count();
        assert_eq!(empty_div_count, 8, "Expected 8 empty hover zone divs");
    }

    #[tokio::test]
    async fn test_hover3d_custom_class() {
        let html = rsx! {
            <Hover3d class="my-12 mx-2">
                <figure>"Content"</figure>
            </Hover3d>
        };
        assert!(html.contains("hover-3d my-12 mx-2"));
    }

    #[tokio::test]
    async fn test_hover3d_total_children() {
        let html = rsx! {
            <Hover3d>
                <img src="a.jpg" />
            </Hover3d>
        };
        // Count all opening div tags: 1 wrapper + 8 empty zones = 9
        let open_div_count = html.matches("<div").count();
        assert_eq!(open_div_count, 9, "Should have 9 div elements total");
    }
}
