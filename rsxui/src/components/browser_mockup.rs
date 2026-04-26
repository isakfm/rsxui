//! Browser Mockup Component
//!
//! Browser mockup shows a box that looks like a browser window.
//!
//! # DaisyUI Classes
//! - Base: `mockup-browser`
//! - Part: `mockup-browser-toolbar`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{BrowserMockup, BrowserToolbar};
//! use rsx::rsx;
//!
//! let html = rsx! {
//!     <BrowserMockup>
//!         <BrowserToolbar>
//!             <div class="input">"https://daisyui.com"</div>
//!         </BrowserToolbar>
//!         <div class="flex justify-center px-4 py-16 bg-base-200">"Hello World!"</div>
//!     </BrowserMockup>
//! };
//! ```

use rsx::{classes, component, rsx};

// ============================================
// BrowserToolbar - Browser address bar
// ============================================

#[component]
pub fn BrowserToolbar(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div class={classes!("mockup-browser-toolbar", class)}>{children}</div>
    }
}

// ============================================
// BrowserMockup - Browser window mockup
// ============================================

#[component]
pub fn BrowserMockup(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div class={classes!("mockup-browser", class)}>{children}</div>
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
    async fn test_browser_mockup_basic() {
        let html = rsx! {
            <BrowserMockup>
                <BrowserToolbar>
                    <div class="input">"https://example.com"</div>
                </BrowserToolbar>
                <div>"Content"</div>
            </BrowserMockup>
        };
        assert!(html.contains(r#"class="mockup-browser""#));
        assert!(html.contains(r#"class="mockup-browser-toolbar""#));
        assert!(html.contains(">Content</div>"));
    }

    #[tokio::test]
    async fn test_browser_mockup_custom_class() {
        let html = rsx! {
            <BrowserMockup class="border">
                <BrowserToolbar class="bg-base-300">"X"</BrowserToolbar>
            </BrowserMockup>
        };
        assert!(html.contains("border"));
        assert!(html.contains("bg-base-300"));
    }
}
