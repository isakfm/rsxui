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
//! use rsx_macros::rsx;
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

use rsx_macros::{classes, component, rsx};

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

#[component]
pub fn Drawer(
    id: String,
    #[builder(default)] placement: DrawerPlacement,
    #[builder(default)] open: bool,
    #[builder(default)] class: String,
    children: String,
) -> String {
    let lg_class = if open { "lg:drawer-open" } else { "" };

    rsx! {
        <div
            class={classes!("drawer", lg_class, placement.as_class(), class)}
            id={id}>
            {children}
        </div>
    }
}

// ============================================
// DrawerToggle - Hidden checkbox for state
// ============================================

#[component]
pub fn DrawerToggle(id: String) -> String {
    rsx! {
        <input id={id} type="checkbox" class="drawer-toggle" />
    }
}

// ============================================
// DrawerContent - Main page content
// ============================================

#[component]
pub fn DrawerContent(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div class={classes!("drawer-content", class)}>
            {children}
        </div>
    }
}

// ============================================
// DrawerSide - Sidebar container
// ============================================

#[component]
pub fn DrawerSide(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div class={classes!("drawer-side", class)}>
            {children}
        </div>
    }
}

// ============================================
// DrawerOverlay - Click to close overlay
// ============================================

#[component]
pub fn DrawerOverlay(for_id: String) -> String {
    rsx! {
        <label for={for_id} aria-label="close sidebar" class="drawer-overlay" />
    }
}

// ============================================
// DrawerButton - Toggle button
// ============================================

#[component]
pub fn DrawerButton(
    for_id: String,
    #[builder(default)] show_on: String,
    #[builder(default)] class: String,
    label: String,
) -> String {
    rsx! {
        <label for={for_id} class={classes!("btn drawer-button", show_on, class)} >{label}</label>
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
