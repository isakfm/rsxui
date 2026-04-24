//! Swap Component
//!
//! Swap allows you to toggle the visibility of two elements using a checkbox or a class name.
//!
//! # DaisyUI Classes
//! - Base: `swap`
//! - Part: `swap-on`, `swap-off`, `swap-indeterminate`
//! - Modifier: `swap-active`
//! - Styles: `swap-rotate`, `swap-flip`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{Swap, SwapOn, SwapOff};
//! use rsx_macros::rsx;
//!
//! let html = rsx! {
//!     <Swap>
//!         <input type="checkbox" />
//!         <SwapOn>"ON"</SwapOn>
//!         <SwapOff>"OFF"</SwapOff>
//!     </Swap>
//! };
//! ```

use rsx_macros::{classes, component, rsx};

use super::class_if;

// ============================================
// SwapOn - Visible when swap is active
// ============================================

#[component]
pub fn SwapOn(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div class={classes!("swap-on", class)}>{children}</div>
    }
}

// ============================================
// SwapOff - Visible when swap is inactive
// ============================================

#[component]
pub fn SwapOff(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div class={classes!("swap-off", class)}>{children}</div>
    }
}

// ============================================
// SwapIndeterminate - Visible when indeterminate
// ============================================

#[component]
pub fn SwapIndeterminate(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div class={classes!("swap-indeterminate", class)}>{children}</div>
    }
}

// ============================================
// Swap - Toggle visibility container
// ============================================

#[component]
pub fn Swap(
    #[builder(default)] active: bool,
    #[builder(default)] rotate: bool,
    #[builder(default)] flip: bool,
    #[builder(default)] class: String,
    children: String,
) -> String {
    rsx! {
        <label class={classes!(
            "swap",
            class_if(active, "swap-active"),
            class_if(rotate, "swap-rotate"),
            class_if(flip, "swap-flip"),
            class,
        )}>
            {children}
        </label>
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
    async fn test_swap_basic() {
        let html = rsx! {
            <Swap>
                <input type="checkbox" />
                <SwapOn>"ON"</SwapOn>
                <SwapOff>"OFF"</SwapOff>
            </Swap>
        };
        assert!(html.contains(r#"class="swap""#));
        assert!(html.contains(r#"class="swap-on""#));
        assert!(html.contains(r#"class="swap-off""#));
        assert!(html.contains(">ON</div>"));
        assert!(html.contains(">OFF</div>"));
    }

    #[tokio::test]
    async fn test_swap_active() {
        let html = rsx! {
            <Swap active=true>"X"</Swap>
        };
        assert!(html.contains("swap-active"));
    }

    #[tokio::test]
    async fn test_swap_rotate() {
        let html = rsx! {
            <Swap rotate=true>"X"</Swap>
        };
        assert!(html.contains("swap-rotate"));
    }

    #[tokio::test]
    async fn test_swap_flip() {
        let html = rsx! {
            <Swap flip=true>"X"</Swap>
        };
        assert!(html.contains("swap-flip"));
    }

    #[tokio::test]
    async fn test_swap_indeterminate() {
        let html = rsx! {
            <SwapIndeterminate>"?"</SwapIndeterminate>
        };
        assert!(html.contains(r#"class="swap-indeterminate""#));
    }

    #[tokio::test]
    async fn test_swap_custom_class() {
        let html = rsx! {
            <Swap class="text-9xl">
                <SwapOn class="font-bold">"A"</SwapOn>
            </Swap>
        };
        assert!(html.contains("text-9xl"));
        assert!(html.contains("font-bold"));
    }
}
