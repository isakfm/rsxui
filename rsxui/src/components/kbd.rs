//! Kbd Component
//!
//! A keyboard key display component with DaisyUI styling.
//!
//! # DaisyUI Classes
//! - Base: `kbd`
//! - Sizes: `kbd-xs`, `kbd-sm`, `kbd-md`, `kbd-lg`, `kbd-xl`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{Kbd, Size};
//! use rsx::rsx;
//!
//! let html = rsx! {
//!     <Kbd size=Size::Lg>"Ctrl"</Kbd>
//! };
//! ```

use rsx::{classes, component, rsx};

use super::Size;

// ============================================
// Kbd - Keyboard key display
// ============================================

#[component]
pub fn Kbd(
    #[builder(default)] size: Size,
    #[builder(default)] class: String,
    children: String,
) -> String {
    rsx! {
        <kbd class={classes!("kbd", size.prefix("kbd"), class)}>
            {children}
        </kbd>
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
    async fn test_kbd_basic() {
        let html = rsx! {
            <Kbd>"K"</Kbd>
        };
        assert!(html.contains("<kbd"));
        assert!(html.contains(r#"class="kbd""#));
        assert!(html.contains(">K</kbd>"));
    }

    #[tokio::test]
    async fn test_kbd_all_sizes() {
        for size in [Size::Xs, Size::Sm, Size::Md, Size::Lg, Size::Xl] {
            let html = rsx! {
                <Kbd size=size>"K"</Kbd>
            };
            let expected = size.prefix("kbd");
            assert!(
                html.contains(&expected),
                "Missing size class {} for {:?}",
                expected,
                size
            );
        }
    }

    #[tokio::test]
    async fn test_kbd_custom_class() {
        let html = rsx! {
            <Kbd class="my-kbd">"K"</Kbd>
        };
        assert!(html.contains("my-kbd"));
    }

    #[tokio::test]
    async fn test_kbd_combined() {
        let html = rsx! {
            <Kbd size=Size::Lg class="font-mono">"Ctrl + C"</Kbd>
        };
        assert!(html.contains("kbd-lg"));
        assert!(html.contains("font-mono"));
        assert!(html.contains("Ctrl + C"));
    }
}
