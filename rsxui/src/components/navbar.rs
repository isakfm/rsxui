//! Navbar Component
//!
//! Navbar is used to show a navigation bar on the top of the page.
//!
//! # DaisyUI Classes
//! - Base: `navbar`
//! - Parts: `navbar-start`, `navbar-center`, `navbar-end`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{Navbar, NavbarStart, NavbarCenter, NavbarEnd};
//! use rsx_macros::rsx;
//!
//! let html = rsx! {
//!     <Navbar>
//!         <NavbarStart>"Logo"</NavbarStart>
//!         <NavbarCenter>"Menu"</NavbarCenter>
//!         <NavbarEnd>"Actions"</NavbarEnd>
//!     </Navbar>
//! };
//! ```

use rsx_macros::{classes, component, rsx};

// ============================================
// NavbarStart - Left-aligned section
// ============================================

#[component]
pub fn NavbarStart(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div class={classes!("navbar-start", class)}>{children}</div>
    }
}

// ============================================
// NavbarCenter - Center-aligned section
// ============================================

#[component]
pub fn NavbarCenter(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div class={classes!("navbar-center", class)}>{children}</div>
    }
}

// ============================================
// NavbarEnd - Right-aligned section
// ============================================

#[component]
pub fn NavbarEnd(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div class={classes!("navbar-end", class)}>{children}</div>
    }
}

// ============================================
// Navbar - Navigation bar container
// ============================================

#[component]
pub fn Navbar(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div class={classes!("navbar", class)}>{children}</div>
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
    async fn test_navbar_basic() {
        let html = rsx! {
            <Navbar>"Content"</Navbar>
        };
        assert!(html.contains(r#"class="navbar""#));
        assert!(html.contains(">Content</div>"));
    }

    #[tokio::test]
    async fn test_navbar_sections() {
        let html = rsx! {
            <Navbar>
                <NavbarStart>"Start"</NavbarStart>
                <NavbarCenter>"Center"</NavbarCenter>
                <NavbarEnd>"End"</NavbarEnd>
            </Navbar>
        };
        assert!(html.contains("navbar-start"));
        assert!(html.contains("navbar-center"));
        assert!(html.contains("navbar-end"));
        assert!(html.contains(">Start</div>"));
        assert!(html.contains(">Center</div>"));
        assert!(html.contains(">End</div>"));
    }

    #[tokio::test]
    async fn test_navbar_custom_class() {
        let html = rsx! {
            <Navbar class="bg-base-200">"Nav"</Navbar>
        };
        assert!(html.contains("navbar bg-base-200"));
    }

    #[tokio::test]
    async fn test_navbar_start_custom_class() {
        let html = rsx! {
            <NavbarStart class="flex-1">"Start"</NavbarStart>
        };
        assert!(html.contains("navbar-start flex-1"));
    }
}
