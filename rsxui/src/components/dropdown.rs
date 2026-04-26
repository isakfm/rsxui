//! Dropdown Component
//!
//! Dropdown can open a menu or any other element when the button is clicked.
//!
//! # DaisyUI Classes
//! - Base: `dropdown`
//! - Part: `dropdown-content`
//! - Placement: `dropdown-start`, `dropdown-center`, `dropdown-end`, `dropdown-top`, `dropdown-bottom`, `dropdown-left`, `dropdown-right`
//! - Modifier: `dropdown-hover`, `dropdown-open`, `dropdown-close`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{Dropdown, DropdownContent, DropdownPlacement, DropdownModifier};
//! use rsx::rsx;
//!
//! let html = rsx! {
//!     <Dropdown placement=DropdownPlacement::End modifier=DropdownModifier::Hover>
//!         <div tabindex="0" role="button" class="btn">"Click"</div>
//!         <DropdownContent>
//!             <ul class="menu">
//!                 <li><a>"Item 1"</a></li>
//!             </ul>
//!         </DropdownContent>
//!     </Dropdown>
//! };
//! ```

use enum_stringify::EnumStringify;
use rsx::{classes, component, rsx};

use super::class_if;

// ============================================
// DropdownPlacement - Horizontal/vertical alignment
// ============================================

#[derive(Debug, Clone, Copy, PartialEq, Default, EnumStringify)]
#[enum_stringify(case = "kebab", prefix = "dropdown-")]
pub enum DropdownPlacement {
    #[default]
    Start,
    Center,
    End,
    Top,
    Bottom,
    Left,
    Right,
}

// ============================================
// DropdownModifier - Behavioral modifiers
// ============================================

#[derive(Debug, Clone, Copy, PartialEq, Default, EnumStringify)]
#[enum_stringify(case = "kebab", prefix = "dropdown-")]
pub enum DropdownModifier {
    #[default]
    #[enum_stringify(rename = "")]
    None,
    Hover,
    Open,
    Close,
}

// ============================================
// DropdownContent - Content part of dropdown
// ============================================

#[component]
pub fn DropdownContent(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div tabindex="-1" class={classes!("dropdown-content", class)}>
            {children}
        </div>
    }
}

// ============================================
// Dropdown - Dropdown container
// ============================================

#[component]
pub fn Dropdown(
    #[builder(default)] placement: DropdownPlacement,
    #[builder(default)] modifier: DropdownModifier,
    #[builder(default)] class: String,
    children: String,
) -> String {
    let placement_str = placement.to_string();
    let modifier_str = modifier.to_string();

    rsx! {
        <div class={classes!(
            "dropdown",
            class_if(!placement_str.is_empty(), &placement_str),
            class_if(!modifier_str.is_empty(), &modifier_str),
            class,
        )}>
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
    use rsx::rsx;

    #[tokio::test]
    async fn test_dropdown_basic() {
        let html = rsx! {
            <Dropdown>
                <button>"Open"</button>
                <DropdownContent>"Menu"</DropdownContent>
            </Dropdown>
        };
        assert!(html.contains("dropdown"));
        assert!(html.contains(r#"class="dropdown-content""#));
        assert!(html.contains(">Open</button>"));
    }

    #[tokio::test]
    async fn test_dropdown_all_placements() {
        for placement in [
            DropdownPlacement::Start,
            DropdownPlacement::Center,
            DropdownPlacement::End,
            DropdownPlacement::Top,
            DropdownPlacement::Bottom,
            DropdownPlacement::Left,
            DropdownPlacement::Right,
        ] {
            let html = rsx! {
                <Dropdown placement=placement>"X"</Dropdown>
            };
            let expected = placement.to_string();
            assert!(
                html.contains(&expected),
                "Missing placement class {} for {:?}",
                expected,
                placement
            );
        }
    }

    #[tokio::test]
    async fn test_dropdown_all_modifiers() {
        for modifier in [
            DropdownModifier::Hover,
            DropdownModifier::Open,
            DropdownModifier::Close,
        ] {
            let html = rsx! {
                <Dropdown modifier=modifier>"X"</Dropdown>
            };
            let expected = modifier.to_string();
            assert!(
                html.contains(&expected),
                "Missing modifier class {} for {:?}",
                expected,
                modifier
            );
        }
    }

    #[tokio::test]
    async fn test_dropdown_default_modifier_empty() {
        let html = rsx! {
            <Dropdown>"X"</Dropdown>
        };
        // Default modifier (None) should not add any class
        assert!(!html.contains("dropdown-hover"));
        assert!(!html.contains("dropdown-open"));
        assert!(!html.contains("dropdown-close"));
    }

    #[tokio::test]
    async fn test_dropdown_combined() {
        let html = rsx! {
            <Dropdown placement=DropdownPlacement::End modifier=DropdownModifier::Hover class="mb-32">
                "X"
            </Dropdown>
        };
        assert!(html.contains("dropdown-end"));
        assert!(html.contains("dropdown-hover"));
        assert!(html.contains("mb-32"));
    }

    #[tokio::test]
    async fn test_dropdown_custom_class() {
        let html = rsx! {
            <Dropdown class="mb-32">"X"</Dropdown>
        };
        assert!(html.contains("mb-32"));
    }
}
