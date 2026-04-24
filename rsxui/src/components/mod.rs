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

// ============================================
// Re-exports
// ============================================

pub mod alert;
pub mod badge;
pub mod button;
pub mod card;
pub mod drawer;
pub mod input;

pub use alert::{Alert, AlertProps};
pub use badge::{Badge, BadgeProps, BadgeStyle};
pub use button::{Button, ButtonModifier, ButtonProps, ButtonStyle, ButtonType};
pub use card::{Card, CardActions, CardBody, CardFigure, CardLayout, CardStyle, CardTitle};
pub use drawer::{
    Drawer, DrawerButton, DrawerContent, DrawerMenu, DrawerMenuItem, DrawerNav, DrawerOverlay,
    DrawerPlacement, DrawerSide, DrawerToggle,
};
pub use input::{Input, InputProps, InputStyle};
