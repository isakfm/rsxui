//! List Component
//!
//! A vertical layout to display information in rows.
//!
//! # DaisyUI Classes
//! - Base: `list`
//! - Part: `list-row`
//! - Modifier: `list-col-wrap`, `list-col-grow`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{List, ListRow};
//! use rsx::rsx;
//!
//! let html = rsx! {
//!     <List>
//!         <ListRow>"Item 1"</ListRow>
//!         <ListRow>"Item 2"</ListRow>
//!     </List>
//! };
//! ```

use rsx::attrs::RenderAttrs;
use rsx::{classes, rsx, ui};

use super::class_if;

// ============================================
// ListRow - A single row in a list
// ============================================

#[ui]
pub fn ListRow(
    #[builder(default)] grow: bool,
    #[builder(default)] wrap: bool,
    #[builder(default)] class: String,
    children: String,
) -> String {
    rsx! {
        <li class={classes!("list-row", class_if(props.grow, "list-col-grow"), class_if(props.wrap, "list-col-wrap"), props.class)} {props.render_attrs()}>
            {props.children}
        </li>
    }
}

// ============================================
// List - Vertical list container
// ============================================

#[ui]
pub fn List(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <ul class={classes!("list", props.class)} {props.render_attrs()}>
            {props.children}
        </ul>
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
    async fn test_list_basic() {
        let html = rsx! {
            <List>
                <ListRow>"Item 1"</ListRow>
                <ListRow>"Item 2"</ListRow>
            </List>
        };
        assert!(html.contains(r#"class="list""#));
        assert!(html.contains(r#"class="list-row""#));
        assert!(html.contains(">Item 1</li>"));
        assert!(html.contains(">Item 2</li>"));
    }

    #[tokio::test]
    async fn test_list_row_grow() {
        let html = rsx! {
            <ListRow grow=true>"Grown"</ListRow>
        };
        assert!(html.contains("list-col-grow"));
    }

    #[tokio::test]
    async fn test_list_row_wrap() {
        let html = rsx! {
            <ListRow wrap=true>"Wrapped"</ListRow>
        };
        assert!(html.contains("list-col-wrap"));
    }

    #[tokio::test]
    async fn test_list_custom_class() {
        let html = rsx! {
            <List class="my-list">
                <ListRow class="my-row">"X"</ListRow>
            </List>
        };
        assert!(html.contains("my-list"));
        assert!(html.contains("my-row"));
    }
}
