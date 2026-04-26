//! RsxUI Core - Core rendering infrastructure for RsxUI
//!
//! This crate provides the foundational types and traits for HTML rendering,
//! including the `Buffer` abstraction, HTML escaping, and attribute handling.

mod buffer;
pub mod elements;
pub mod error;
pub mod escape;
pub mod props;
pub mod render;

pub mod attributes;
pub mod attrs;
pub mod context;
pub mod prelude;

// Re-export proc macros so downstream crates can depend on `rsx` only.
pub use rsx_macros::{classes, component, props, raw, rsx, ui};

pub use attributes::EscapeAttribute;
pub use buffer::Buffer;
pub use elements::Elements;
pub use error::RsxuiError;
pub use escape::{escape_attribute, escape_content, PreEscaped};
pub use props::{props_builder, Component, Props};
pub use render::Render;

pub use bon;
pub use html_escape::encode_unquoted_attribute;
