//! Theme Controller Component
//!
//! Theme Controller changes the page theme using CSS only.
//! When a checked input with `theme-controller` class exists,
//! the page uses that input's value as the theme.
//!
//! # DaisyUI Classes
//! - Controller: `theme-controller`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{ThemeControllerToggle, ThemeControllerRadio};
//! use rsx_macros::rsx;
//!
//! let html = rsx! {
//!     <ThemeControllerToggle value="synthwave" />
//!     <ThemeControllerRadio name="theme-radios" value="retro" />
//! };
//! ```

use rsx_macros::{classes, component, rsx};

use crate::components::{attr_if, show_if};

// ============================================
// ThemeControllerToggle - Toggle switch for theme
// ============================================

#[component]
pub fn ThemeControllerToggle(
    value: String,
    #[builder(default)] checked: bool,
    #[builder(default)] class: String,
) -> String {
    rsx! {
        <input
            type="checkbox"
            value={value}
            class={classes!("toggle", "theme-controller", class)}
            {show_if(checked, "checked")}
        />
    }
}

// ============================================
// ThemeControllerCheckbox - Checkbox for theme
// ============================================

#[component]
pub fn ThemeControllerCheckbox(
    value: String,
    #[builder(default)] checked: bool,
    #[builder(default)] class: String,
) -> String {
    rsx! {
        <input
            type="checkbox"
            value={value}
            class={classes!("checkbox", "theme-controller", class)}
            {show_if(checked, "checked")}
        />
    }
}

// ============================================
// ThemeControllerRadio - Radio button for theme
// ============================================

#[component]
pub fn ThemeControllerRadio(
    name: String,
    value: String,
    #[builder(default)] checked: bool,
    #[builder(default)] class: String,
) -> String {
    rsx! {
        <input
            type="radio"
            name={name}
            value={value}
            class={classes!("radio", "theme-controller", class)}
            {show_if(checked, "checked")}
        />
    }
}

// ============================================
// ThemeControllerButton - Button-style radio for theme
// ============================================

#[component]
pub fn ThemeControllerButton(
    name: String,
    value: String,
    #[builder(into)] aria_label: Option<String>,
    #[builder(default)] checked: bool,
    #[builder(default)] class: String,
) -> String {
    rsx! {
        <input
            type="radio"
            name={name}
            value={value}
            class={classes!("btn", "theme-controller", class)}
            {attr_if("aria-label", &aria_label)}
            {show_if(checked, "checked")}
        />
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
    async fn test_theme_controller_toggle() {
        let html = rsx! {
            <ThemeControllerToggle value="synthwave" />
        };
        assert!(html.contains(r#"type="checkbox""#));
        assert!(html.contains(r#"value="synthwave""#));
        assert!(html.contains("toggle theme-controller"));
    }

    #[tokio::test]
    async fn test_theme_controller_toggle_checked() {
        let html = rsx! {
            <ThemeControllerToggle value="dark" checked=true />
        };
        assert!(html.contains(" checked"));
    }

    #[tokio::test]
    async fn test_theme_controller_checkbox() {
        let html = rsx! {
            <ThemeControllerCheckbox value="retro" />
        };
        assert!(html.contains(r#"type="checkbox""#));
        assert!(html.contains(r#"value="retro""#));
        assert!(html.contains("checkbox theme-controller"));
    }

    #[tokio::test]
    async fn test_theme_controller_checkbox_checked() {
        let html = rsx! {
            <ThemeControllerCheckbox value="retro" checked=true />
        };
        assert!(html.contains(" checked"));
    }

    #[tokio::test]
    async fn test_theme_controller_radio() {
        let html = rsx! {
            <ThemeControllerRadio name="theme-radios" value="cyberpunk" />
        };
        assert!(html.contains(r#"type="radio""#));
        assert!(html.contains(r#"name="theme-radios""#));
        assert!(html.contains(r#"value="cyberpunk""#));
        assert!(html.contains("radio theme-controller"));
    }

    #[tokio::test]
    async fn test_theme_controller_radio_checked() {
        let html = rsx! {
            <ThemeControllerRadio name="theme-radios" value="aqua" checked=true />
        };
        assert!(html.contains(" checked"));
    }

    #[tokio::test]
    async fn test_theme_controller_button() {
        let html = rsx! {
            <ThemeControllerButton name="theme-buttons" value="valentine" aria_label="Valentine" />
        };
        assert!(html.contains(r#"type="radio""#));
        assert!(html.contains(r#"name="theme-buttons""#));
        assert!(html.contains(r#"value="valentine""#));
        assert!(html.contains(r#"aria-label="Valentine""#));
        assert!(html.contains("btn theme-controller"));
    }

    #[tokio::test]
    async fn test_theme_controller_custom_class() {
        let html = rsx! {
            <ThemeControllerToggle value="dark" class="bg-primary" />
        };
        assert!(html.contains("toggle theme-controller bg-primary"));
    }

    #[tokio::test]
    async fn test_theme_controller_button_join_item() {
        let html = rsx! {
            <ThemeControllerButton
                name="theme-buttons"
                value="retro"
                aria_label="Retro"
                class="join-item"
            />
        };
        assert!(html.contains("btn theme-controller join-item"));
    }
}
