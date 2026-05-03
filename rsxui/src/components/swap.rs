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
//! use rsx::rsx;
//!
//! let html = rsx! {
//!     <Swap>
//!         <input type="checkbox" />
//!         <SwapOn>"ON"</SwapOn>
//!         <SwapOff>"OFF"</SwapOff>
//!     </Swap>
//! };
//! ```

use rsx::attrs::RenderAttrs;
use rsx::{classes, rsx, ui};

use super::class_if;

// ============================================
// SwapOn - Visible when swap is active
// ============================================

#[ui]
pub fn SwapOn(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div class={classes!("swap-on", props.class)} {props.render_attrs()}>{props.children}</div>
    }
}

// ============================================
// SwapOff - Visible when swap is inactive
// ============================================

#[ui]
pub fn SwapOff(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div class={classes!("swap-off", props.class)} {props.render_attrs()}>{props.children}</div>
    }
}

// ============================================
// SwapIndeterminate - Visible when indeterminate
// ============================================

#[ui]
pub fn SwapIndeterminate(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div class={classes!("swap-indeterminate", props.class)} {props.render_attrs()}>{props.children}</div>
    }
}

// ============================================
// Swap - Toggle visibility container
// ============================================

#[ui]
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
            class_if(props.active, "swap-active"),
            class_if(props.rotate, "swap-rotate"),
            class_if(props.flip, "swap-flip"),
            props.class,
        )} {props.render_attrs()}>
            {props.children}
        </label>
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
