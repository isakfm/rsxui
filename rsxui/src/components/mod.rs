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

/// Helper: returns a boolean attribute (e.g. `disabled`, `checked`, `open`) when condition is true,
/// otherwise returns empty string. The attribute name is HTML-escaped.
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
pub mod breadcrumb;
pub mod browser_mockup;
pub mod button;
pub mod card;
pub mod chat;
pub mod checkbox;
pub mod code_mockup;
pub mod collapse;
pub mod countdown;
pub mod diff;
pub mod divider;
pub mod dock;
pub mod drawer;
pub mod dropdown;
pub mod fieldset;
pub mod file_input;
pub mod filter;
pub mod footer;
pub mod hero;
pub mod hover3d;
pub mod hover_gallery;
pub mod indicator;
pub mod input;
pub mod join;
pub mod kbd;
pub mod label;
pub mod link;
pub mod list;
pub mod loading;
pub mod mask;
pub mod menu;
pub mod navbar;
pub mod pagination;
pub mod phone_mockup;
pub mod progress;
pub mod radial_progress;
pub mod radio;
pub mod range;
pub mod rating;
pub mod select;
pub mod skeleton;
pub mod stack;
pub mod stat;
pub mod status;
pub mod steps;
pub mod swap;
pub mod tab;
pub mod table;
pub mod textarea;
pub mod textrotate;
pub mod timeline;
pub mod toast;
pub mod toggle;
pub mod tooltip;
pub mod validator;
pub mod window_mockup;

pub use accordion::{AccordionDetails, AccordionItem, AccordionModifier};
pub use alert::{Alert, AlertProps};
pub use avatar::{Avatar, AvatarGroup, AvatarStatus};
pub use badge::{Badge, BadgeProps, BadgeStyle};
pub use breadcrumb::Breadcrumb;
pub use browser_mockup::{BrowserMockup, BrowserToolbar};
pub use button::{Button, ButtonModifier, ButtonProps, ButtonStyle, ButtonType};
pub use card::{Card, CardActions, CardBody, CardFigure, CardLayout, CardStyle, CardTitle};
pub use chat::{Chat, ChatBubble, ChatFooter, ChatHeader, ChatImage, ChatPlacement};
pub use checkbox::Checkbox;
pub use code_mockup::CodeMockup;
pub use collapse::{Collapse, CollapseContent, CollapseModifier, CollapseTitle};
pub use countdown::{Countdown, CountdownGroup, CountdownValue};
pub use diff::{Diff, DiffItem1, DiffItem2, DiffResizer};
pub use divider::Divider;
pub use dock::{Dock, DockItem};
pub use drawer::{
    Drawer, DrawerButton, DrawerContent, DrawerOverlay, DrawerPlacement, DrawerSide, DrawerToggle,
};
pub use dropdown::{Dropdown, DropdownContent, DropdownModifier, DropdownPlacement};
pub use fieldset::Fieldset;
pub use file_input::{FileInput, FileInputProps, FileInputStyle};
pub use filter::Filter;
pub use footer::{Footer, FooterTitle};
pub use hero::{Hero, HeroContent, HeroOverlay};
pub use hover3d::Hover3d;
pub use hover_gallery::HoverGallery;
pub use indicator::{Indicator, IndicatorItem};
pub use input::{Input, InputProps, InputStyle};
pub use join::{Join, JoinItem};
pub use kbd::Kbd;
pub use label::{FloatingLabel, Label};
pub use link::{Link, LinkProps, LinkStyle};
pub use list::{List, ListRow};
pub use loading::{Loading, LoadingStyle};
pub use mask::{Mask, MaskModifier, MaskShape};
pub use menu::{Menu, MenuItem, MenuState};
pub use navbar::{Navbar, NavbarCenter, NavbarEnd, NavbarStart};
pub use pagination::{Pagination, PaginationItem};
pub use phone_mockup::PhoneMockup;
pub use progress::Progress;
pub use radial_progress::RadialProgress;
pub use radio::Radio;
pub use range::Range;
pub use rating::Rating;
pub use select::{Select, SelectProps, SelectStyle};
pub use skeleton::Skeleton;
pub use stack::{Stack, StackModifier};
pub use stat::{Stat, StatActions, StatDesc, StatFigure, StatTitle, StatValue, Stats};
pub use status::Status;
pub use steps::{Step, StepDirection, Steps};
pub use swap::{Swap, SwapIndeterminate, SwapOff, SwapOn};
pub use tab::{Tab, TabContent, TabPlacement, TabRadio, TabStyle, Tabs};
pub use table::Table;
pub use textarea::{Textarea, TextareaProps, TextareaStyle};
pub use textrotate::TextRotate;
pub use timeline::{
    Timeline, TimelineDirection, TimelineEnd, TimelineItem, TimelineMiddle, TimelineModifier,
    TimelineStart,
};
pub use toast::{Toast, ToastHorizontal, ToastVertical};
pub use toggle::Toggle;
pub use tooltip::{Tooltip, TooltipPlacement};
pub use validator::{Validator, ValidatorHint};
pub use window_mockup::WindowMockup;
