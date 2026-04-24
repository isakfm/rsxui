// Menu Components
// Based on DaisyUI Menu: https://daisyui.com/components/menu/

use crate::components::{class_if, show_if};
use enum_stringify::EnumStringify;
use rsx_macros::{classes, component, rsx};

// ============================================
// MenuState - Menu item state
// ============================================

#[derive(Debug, Clone, PartialEq, Default, EnumStringify)]
#[enum_stringify(case = "kebab", prefix = "menu-")]
pub enum MenuState {
    #[default]
    #[enum_stringify(rename = "")]
    Normal,
    Focus,
    Active,
    Disabled,
}

// ============================================
// Menu - Dropdown/Select menu list
// ============================================

#[component]
pub fn Menu(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <ul class={classes!("menu", class)}>
            {children}
        </ul>
    }
}

// ============================================
// MenuItem - Menu list item
// ============================================

#[allow(clippy::unnecessary_braces_for_condition)]
#[component]
pub fn MenuItem(#[builder(default)] state: MenuState, url: String, children: String) -> String {
    if state != MenuState::Normal {
        rsx! {
            <li class={state.to_string()}>
                <button
                {show_if(state==MenuState::Disabled, "disabled")}
                class={class_if(state==MenuState::Focus, "menu-focus")}
                >{children}</button>
            </li>
        }
    } else {
        rsx! {
            <li><a href={url}>{children}</a></li>
        }
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
    async fn test_menu_basic() {
        let html = rsx! {
            <Menu>""</Menu>
        };
        assert!(html.contains("class=\"menu\""));
    }

    #[tokio::test]
    async fn test_menu_item() {
        let html = rsx! {
            <Menu>
                <MenuItem>"Item 1"</MenuItem>
                <MenuItem state=MenuState::Active>"Item 2"</MenuItem>
                <MenuItem state=MenuState::Disabled>"Item 3"</MenuItem>
                <MenuItem state=MenuState::Focus>"Item 4"</MenuItem>
            </Menu>
        };
        assert!(html.contains("class=\"menu\""));
        assert!(html.contains("Item 1"));
        assert!(html.contains("active"));
        assert!(html.contains("disabled"));
        assert!(html.contains("focus"));
    }
}
