//! RsxUI Core - Core rendering infrastructure for RsxUI
//!
//! This crate provides the foundational types and traits for HTML rendering,
//! including the `Buffer` abstraction, HTML escaping, and attribute handling.

mod buffer;
pub mod elements;
pub mod escape;
pub mod error;
pub mod props;
pub mod render;

pub mod attributes;
pub mod attrs;
pub mod context;
pub mod prelude;

pub use buffer::Buffer;
pub use elements::Elements;
pub use escape::{escape_content, escape_attribute, PreEscaped};
pub use error::RsxuiError;
pub use props::{Component, Props, props_builder};
pub use render::Render;
pub use attributes::EscapeAttribute;

pub use html_escape::encode_unquoted_attribute;
pub use bon;
