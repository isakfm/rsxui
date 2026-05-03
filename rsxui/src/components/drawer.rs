//! Drawer Component
//!
//! A responsive side drawer/navigation component with DaisyUI styling.
//!
//! # DaisyUI Classes
//! - Base: `drawer`
//! - Parts: `drawer-toggle`, `drawer-content`, `drawer-side`, `drawer-overlay`
//! - Placement: `drawer-end` (right side)
//! - Modifiers: `lg:drawer-open` (visible on large screens)
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{Drawer, DrawerSide, Color};
//! use rsx::rsx;
//!
//! let html = rsx! {
//!     <Drawer id="my-drawer">
//!         <DrawerContent>
//!             <p>"Main content"</p>
//!         </DrawerContent>
//!         <DrawerSide>
//!             <DrawerOverlay />
//!             <ul class="menu">
//!                 <li><a>"Home"</a></li>
//!                 <li><a>"About"</a></li>
//!             </ul>
//!         </DrawerSide>
//!     </Drawer>
//! };
//! ```

use rsx::attrs::RenderAttrs;
use rsx::{classes, rsx, ui};

#[allow(unused_imports)]
use super::{Menu, MenuItem, MenuState};

// ============================================
// DrawerPlacement - Side placement
// ============================================

#[derive(Debug, Clone, PartialEq, Default)]
pub enum DrawerPlacement {
    #[default]
    Left,
    Right,
}

impl DrawerPlacement {
    pub fn as_class(&self) -> &'static str {
        match self {
            DrawerPlacement::Left => "",
            DrawerPlacement::Right => "drawer-end",
        }
    }
}

// ============================================
// Drawer - Main wrapper
// ============================================

#[ui]
pub fn Drawer(
    #[builder(default)] placement: DrawerPlacement,
    #[builder(default)] open: bool,
    #[builder(default)] class: String,
    children: String,
) -> String {
    let lg_class = if props.open { "lg:drawer-open" } else { "" };

    rsx! {
        <div
            class={classes!("drawer", lg_class, props.placement.as_class(), props.class)}
            {props.render_attrs()}>
            {props.children}
        </div>
    }
}

// ============================================
// DrawerToggle - Hidden checkbox for state
// ============================================

#[ui]
pub fn DrawerToggle() -> String {
    rsx! {
        <input type="checkbox" class="drawer-toggle"  {props.render_attrs()}/>
    }
}

// ============================================
// DrawerContent - Main page content
// ============================================

#[ui]
pub fn DrawerContent(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div class={classes!("drawer-content", props.class)} {props.render_attrs()}>
            {props.children}
        </div>
    }
}

// ============================================
// DrawerSide - Sidebar container
// ============================================

#[ui]
pub fn DrawerSide(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div class={classes!("drawer-side", props.class)} {props.render_attrs()}>
            {props.children}
        </div>
    }
}

// ============================================
// DrawerOverlay - Click to close overlay
// ============================================

#[ui]
pub fn DrawerOverlay(for_id: String) -> String {
    rsx! {
        <label for={props.for_id.clone()} aria-label="close sidebar" class="drawer-overlay"  {props.render_attrs()}/>
    }
}

// ============================================
// DrawerButton - Toggle button
// ============================================

#[ui]
pub fn DrawerButton(
    for_id: String,
    #[builder(default)] show_on: String,
    #[builder(default)] class: String,
    label: String,
) -> String {
    rsx! {
        <label for={props.for_id.clone()} class={classes!("btn drawer-button", props.show_on.clone(), props.class)}  {props.render_attrs()}>{props.label.clone()}</label>
    }
}

// ============================================
// Tests
// ============================================

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_drawer_basic() {
        let html = rsx! {
            <Drawer id="test-drawer">
                <DrawerContent>"Content"</DrawerContent>
                <DrawerSide>
                    <DrawerOverlay for_id="test-drawer" />
                    <ul class="menu"><li>"Item"</li></ul>
                </DrawerSide>
            </Drawer>
        };
        assert!(html.contains("class=\"drawer\""));
        assert!(html.contains("drawer-content"));
        assert!(html.contains("drawer-side"));
        assert!(html.contains("drawer-overlay"));
    }

    #[tokio::test]
    async fn test_drawer_toggle() {
        let html = rsx! {
            <DrawerToggle id="my-drawer" />
        };
        assert!(html.contains("type=\"checkbox\""));
        assert!(html.contains("class=\"drawer-toggle\""));
        assert!(html.contains("id=\"my-drawer\""));
    }

    #[tokio::test]
    async fn test_drawer_right_placement() {
        let html = rsx! {
            <Drawer id="right" placement=DrawerPlacement::Right>
                <DrawerContent>"Content"</DrawerContent>
            </Drawer>
        };
        assert!(html.contains("drawer-end"));
    }

    #[tokio::test]
    async fn test_drawer_lg_open() {
        let html = rsx! {
            <Drawer id="test" open=true>
                <DrawerContent>"Content"</DrawerContent>
            </Drawer>
        };
        assert!(html.contains("lg:drawer-open"));
    }

    #[tokio::test]
    async fn test_drawer_button() {
        let html = rsx! {
            <DrawerButton for_id="my-drawer" show_on="lg:hidden" label="Open" />
        };
        assert!(html.contains("Open"));
        assert!(html.contains("btn drawer-button"));
        assert!(html.contains("lg:hidden"));
    }

    #[tokio::test]
    async fn test_drawer_menu() {
        let html = rsx! {
            <Menu class="p-4 w-80 min-h-full bg-base-100 text-base-content">
                <MenuItem>"Home"</MenuItem>
                <MenuItem>"About"</MenuItem>
            </Menu>
        };
        assert!(html.contains("class=\"menu"));
        assert!(html.contains("w-80"));
        assert!(html.contains("Home"));
        assert!(html.contains("About"));
    }

    #[tokio::test]
    async fn test_drawer_overlay() {
        let html = rsx! {
            <DrawerOverlay for_id="my-drawer" />
        };
        assert!(html.contains("label"));
        assert!(html.contains("for=\"my-drawer\""));
        assert!(html.contains("drawer-overlay"));
        assert!(html.contains("aria-label=\"close sidebar\""));
    }

    #[tokio::test]
    async fn test_drawer_full_example() {
        let html = rsx! {
            <Drawer id="main-drawer" open=true>
                <DrawerToggle id="main-drawer" />
                <DrawerContent>
                    <DrawerButton for_id="main-drawer" show_on="lg:hidden" />
                    <p>"Main content here"</p>
                </DrawerContent>
                <DrawerSide>
                    <DrawerOverlay for_id="main-drawer" />
                    <Menu class="p-4 w-80 min-h-full bg-base-100 text-base-content">
                        <MenuItem>"Dashboard"</MenuItem>
                        <MenuItem>"Settings"</MenuItem>
                    </Menu>
                </DrawerSide>
            </Drawer>
        };
        assert!(html.contains("drawer"));
        assert!(html.contains("lg:drawer-open"));
        assert!(html.contains("drawer-toggle"));
        assert!(html.contains("drawer-content"));
        assert!(html.contains("drawer-side"));
        assert!(html.contains("drawer-overlay"));
        assert!(html.contains("menu"));
        assert!(html.contains("Dashboard"));
        assert!(html.contains("Settings"));
    }
}
