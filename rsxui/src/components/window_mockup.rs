//! Window Mockup Component
//!
//! Window mockup shows a box that looks like an operating system window.
//!
//! # DaisyUI Classes
//! - Base: `mockup-window`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::WindowMockup;
//! use rsx_macros::rsx;
//!
//! let html = rsx! {
//!     <WindowMockup>
//!         <div class="flex justify-center px-4 py-16 bg-base-200">"Hello World!"</div>
//!     </WindowMockup>
//! };
//! ```

use rsx_macros::{classes, component, rsx};

// ============================================
// WindowMockup - OS window mockup
// ============================================

#[component]
pub fn WindowMockup(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div class={classes!("mockup-window", class)}>
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
    async fn test_window_mockup_basic() {
        let html = rsx! {
            <WindowMockup>
                <div>"Hello World!"</div>
            </WindowMockup>
        };
        assert!(html.contains(r#"class="mockup-window""#));
        assert!(html.contains(">Hello World!</div>"));
    }

    #[tokio::test]
    async fn test_window_mockup_custom_class() {
        let html = rsx! {
            <WindowMockup class="border">"X"</WindowMockup>
        };
        assert!(html.contains("border"));
    }
}
