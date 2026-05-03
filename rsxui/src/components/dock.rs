//! Dock Component
//!
//! Dock (also known as Bottom navigation) sticks to the bottom of the screen.
//!
//! # DaisyUI Classes
//! - Base: `dock`
//! - Part: `dock-label`
//! - Modifier: `dock-active`
//! - Sizes: `dock-xs`, `dock-sm`, `dock-md`, `dock-lg`, `dock-xl`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{Dock, DockItem};
//! use rsx::rsx;
//!
//! let html = rsx! {
//!     <Dock>
//!         <DockItem active=true label="Home">"🏠"</DockItem>
//!         <DockItem label="Settings">"⚙️"</DockItem>
//!     </Dock>
//! };
//! ```

use rsx::attrs::RenderAttrs;
use rsx::{classes, rsx, ui};

use super::{Size, class_if};

// ============================================
// DockItem - A single item in the dock
// ============================================

#[ui]
pub fn DockItem(
    #[builder(default)] active: bool,
    #[builder(into)] label: Option<String>,
    #[builder(default)] class: String,
    children: String,
) -> String {
    let label_html = if let Some(ref l) = props.label {
        rsx! { <span class="dock-label">{l.clone()}</span> }
    } else {
        String::new()
    };

    rsx! {
        <button class={classes!(class_if(props.active, "dock-active"), props.class)}>
            {props.children}
            {label_html}
        </button>
    }
}

// ============================================
// Dock - Bottom navigation container
// ============================================

#[ui]
pub fn Dock(
    #[builder(default)] size: Size,
    #[builder(default)] class: String,
    children: String,
) -> String {
    rsx! {
        <div class={classes!("dock", props.size.prefix("dock"), props.class)} {props.render_attrs()}>
            {props.children}
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
    async fn test_dock_basic() {
        let html = rsx! {
            <Dock>
                <DockItem label="Home">"🏠"</DockItem>
                <DockItem active=true label="Inbox">"📥"</DockItem>
            </Dock>
        };
        assert!(html.contains(r#"class="dock""#));
        assert!(html.contains(r#"class="dock-label""#));
        assert!(html.contains(">Home</span>"));
        assert!(html.contains("dock-active"));
    }

    #[tokio::test]
    async fn test_dock_all_sizes() {
        for size in [Size::Xs, Size::Sm, Size::Md, Size::Lg, Size::Xl] {
            let html = rsx! {
                <Dock size=size>"X"</Dock>
            };
            let expected = size.prefix("dock");
            assert!(
                html.contains(&expected),
                "Missing size class {} for {:?}",
                expected,
                size
            );
        }
    }

    #[tokio::test]
    async fn test_dock_item_without_label() {
        let html = rsx! {
            <DockItem>"🏠"</DockItem>
        };
        assert!(!html.contains("dock-label"));
    }

    #[tokio::test]
    async fn test_dock_custom_class() {
        let html = rsx! {
            <Dock class="bg-neutral">"X"</Dock>
        };
        assert!(html.contains("bg-neutral"));
    }
}
