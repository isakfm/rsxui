//! DaisyUI Components
//!
//! Reusable DaisyUI component library.
//!
//! # Shared Types
//!
//! - `Color` - Semantic colors (Primary, Secondary, etc.)
//! - `Size` - Reusable sizes (Xs, Sm, Md, Lg, Xl)
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{Button, Color, Size};
//! use rsx_macros::rsx;
//!
//! let html = rsx! {
//!     <Button label="Click me" color={Color::Primary} size={Size::Lg} />
//! };
//! ```
//!
//! # Components
//!
//! - `Button` / `ButtonProps` - DaisyUI button
//! - `Input` / `InputProps` - DaisyUI input

use std::fmt;

// ============================================
// Color - Shared color enum
// ============================================

/// DaisyUI Color System
///
/// First-class support for DaisyUI semantic colors.
/// Colors can be used with any prefix (e.g., "btn-primary", "alert-error").

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Color {
    Primary,
    Secondary,
    Accent,
    Neutral,
    Base100,
    Base200,
    Base300,
    BaseContent,
    Info,
    Success,
    Warning,
    Error,
    #[default]
    Default,
}

impl Color {
    pub fn as_class(&self) -> &'static str {
        match self {
            Color::Primary => "primary",
            Color::Secondary => "secondary",
            Color::Accent => "accent",
            Color::Neutral => "neutral",
            Color::Base100 => "base-100",
            Color::Base200 => "base-200",
            Color::Base300 => "base-300",
            Color::BaseContent => "base-content",
            Color::Info => "info",
            Color::Success => "success",
            Color::Warning => "warning",
            Color::Error => "error",
            Color::Default => "",
        }
    }

    pub fn prefix(&self, prefix: &str) -> String {
        let class = self.as_class();
        if class.is_empty() {
            return class.into();
        }
        format!("{}-{}", prefix, class)
    }
}

// ============================================
// Size - Shared size enum
// ============================================

/// DaisyUI Size System
///
/// Reusable size enums for DaisyUI components.
/// Sizes can be used with any prefix (e.g., "btn-lg", "badge-sm").

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Size {
    #[default]
    Default,
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

impl Size {
    pub fn as_class(&self) -> &'static str {
        match self {
            Size::Default => "",
            Size::Xs => "xs",
            Size::Sm => "sm",
            Size::Md => "md",
            Size::Lg => "lg",
            Size::Xl => "xl",
        }
    }

    pub fn prefix(&self, prefix: &str) -> String {
        let class = self.as_class();
        if class.is_empty() {
            return class.to_string();
        }
        format!("{}-{}", prefix, class)
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_class())
    }
}

/// Helper: returns `class` if `condition` is true, otherwise empty string.
pub fn class_if(condition: bool, class: &str) -> String {
    if condition {
        class.to_string()
    } else {
        String::new()
    }
}

/// Helper: returns `name="value"` if `value` is `Some`, otherwise empty string.
/// Value is HTML-escaped via `EscapeAttribute`.
pub fn attr_if(name: &str, value: &Option<String>) -> String {
    use rsx::EscapeAttribute;
    value
        .as_ref()
        .map(|v| format!(r#" {}="{}""#, name, v.escape_attribute()))
        .unwrap_or_default()
}

pub fn show_if(condition: bool, attr: &str) -> String {
    use rsx::EscapeAttribute;
    if condition {
        return format!(r#" {}"#, attr.escape_attribute());
    }
    String::new()
}

// ============================================
// Re-exports
// ============================================

pub mod accordion;
pub mod alert;
pub mod avatar;
pub mod badge;
pub mod button;
pub mod card;
pub mod chat;
pub mod collapse;
pub mod countdown;
pub mod drawer;
pub mod input;
pub mod menu;

pub use accordion::{AccordionDetails, AccordionItem, AccordionModifier};
pub use alert::{Alert, AlertProps};
pub use avatar::{Avatar, AvatarGroup, AvatarStatus};
pub use badge::{Badge, BadgeProps, BadgeStyle};
pub use button::{Button, ButtonModifier, ButtonProps, ButtonStyle, ButtonType};
pub use card::{Card, CardActions, CardBody, CardFigure, CardLayout, CardStyle, CardTitle};
pub use chat::{
    Chat, ChatBubble, ChatFooter, ChatHeader, ChatImage, ChatPlacement,
};
pub use collapse::{
    Collapse, CollapseContent, CollapseModifier, CollapseTitle,
};
pub use countdown::{Countdown, CountdownGroup, CountdownValue};
pub use drawer::{
    Drawer, DrawerButton, DrawerContent, DrawerOverlay, DrawerPlacement, DrawerSide, DrawerToggle,
};
pub use input::{Input, InputProps, InputStyle};
pub use menu::{Menu, MenuItem, MenuState};
