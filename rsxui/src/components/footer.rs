//! Footer Component
//!
//! Footer can contain logo, copyright notice, and links to other pages.
//!
//! # DaisyUI Classes
//! - Base: `footer`
//! - Part: `footer-title`
//! - Direction: `footer-horizontal`, `footer-vertical`
//! - Placement: `footer-center`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{Footer, FooterTitle};
//! use rsx_macros::rsx;
//!
//! let html = rsx! {
//!     <Footer>
//!         <nav>
//!             <FooterTitle>"Services"</FooterTitle>
//!             <a class="link link-hover">"Branding"</a>
//!         </nav>
//!     </Footer>
//! };
//! ```

use rsx_macros::{classes, component, rsx};

use super::class_if;

// ============================================
// FooterTitle - Title for a footer nav section
// ============================================

#[component]
pub fn FooterTitle(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <h6 class={classes!("footer-title", class)}>{children}</h6>
    }
}

// ============================================
// Footer - Page footer container
// ============================================

#[component]
pub fn Footer(
    #[builder(default)] vertical: bool,
    #[builder(default)] center: bool,
    #[builder(default)] class: String,
    children: String,
) -> String {
    rsx! {
        <footer class={classes!(
            "footer",
            class_if(vertical, "footer-vertical"),
            class_if(center, "footer-center"),
            class,
        )}>
            {children}
        </footer>
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
    async fn test_footer_basic() {
        let html = rsx! {
            <Footer>
                <nav>
                    <FooterTitle>"Company"</FooterTitle>
                    <a>"About"</a>
                </nav>
            </Footer>
        };
        assert!(html.contains(r#"class="footer""#));
        assert!(html.contains(r#"class="footer-title""#));
        assert!(html.contains(">Company</h6>"));
        assert!(html.contains(">About</a>"));
    }

    #[tokio::test]
    async fn test_footer_vertical() {
        let html = rsx! {
            <Footer vertical=true>"X"</Footer>
        };
        assert!(html.contains("footer-vertical"));
    }

    #[tokio::test]
    async fn test_footer_center() {
        let html = rsx! {
            <Footer center=true>"X"</Footer>
        };
        assert!(html.contains("footer-center"));
    }

    #[tokio::test]
    async fn test_footer_custom_class() {
        let html = rsx! {
            <Footer class="bg-base-200 p-10">
                <FooterTitle class="text-primary">"Title"</FooterTitle>
            </Footer>
        };
        assert!(html.contains("bg-base-200"));
        assert!(html.contains("p-10"));
        assert!(html.contains("text-primary"));
    }
}
