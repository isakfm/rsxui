//! Table Component
//!
//! Table can be used to show a list of data in a table format.
//!
//! # DaisyUI Classes
//! - Base: `table`
//! - Modifiers: `table-zebra`, `table-pin-rows`, `table-pin-cols`
//! - Sizes: `table-xs`, `table-sm`, `table-md`, `table-lg`, `table-xl`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::Table;
//! use rsx_macros::rsx;
//!
//! let html = rsx! {
//!     <Table zebra=true>
//!         <thead><tr><th>"Name"</th></tr></thead>
//!         <tbody><tr><td>"Alice"</td></tr></tbody>
//!     </Table>
//! };
//! ```

use rsx_macros::{classes, component, rsx};

use super::{class_if, Size};

// ============================================
// Table - Data table
// ============================================

#[component]
pub fn Table(
    #[builder(default)] zebra: bool,
    #[builder(default)] pin_rows: bool,
    #[builder(default)] pin_cols: bool,
    #[builder(default)] size: Size,
    #[builder(default)] class: String,
    children: String,
) -> String {
    rsx! {
        <div class="overflow-x-auto">
            <table class={classes!(
                "table",
                class_if(zebra, "table-zebra"),
                class_if(pin_rows, "table-pin-rows"),
                class_if(pin_cols, "table-pin-cols"),
                size.prefix("table"),
                class,
            )}>
                {children}
            </table>
        </div>
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
    async fn test_table_basic() {
        let html = rsx! {
            <Table>
                <thead><tr><th>"Name"</th></tr></thead>
                <tbody><tr><td>"Alice"</td></tr></tbody>
            </Table>
        };
        assert!(html.contains(r#"class="table""#));
        assert!(html.contains("<thead>"));
        assert!(html.contains("<tbody>"));
        assert!(html.contains(">Name</th>"));
        assert!(html.contains(">Alice</td>"));
    }

    #[tokio::test]
    async fn test_table_zebra() {
        let html = rsx! {
            <Table zebra=true><tbody><tr><td>"A"</td></tr></tbody></Table>
        };
        assert!(html.contains("table-zebra"));
    }

    #[tokio::test]
    async fn test_table_pin_rows() {
        let html = rsx! {
            <Table pin_rows=true><tbody><tr><td>"A"</td></tr></tbody></Table>
        };
        assert!(html.contains("table-pin-rows"));
    }

    #[tokio::test]
    async fn test_table_pin_cols() {
        let html = rsx! {
            <Table pin_cols=true><tbody><tr><td>"A"</td></tr></tbody></Table>
        };
        assert!(html.contains("table-pin-cols"));
    }

    #[tokio::test]
    async fn test_table_size() {
        let html = rsx! {
            <Table size=Size::Xs><tbody><tr><td>"A"</td></tr></tbody></Table>
        };
        assert!(html.contains("table-xs"));
    }

    #[tokio::test]
    async fn test_table_custom_class() {
        let html = rsx! {
            <Table class="bg-base-200"><tbody><tr><td>"A"</td></tr></tbody></Table>
        };
        assert!(html.contains("bg-base-200"));
    }

    #[tokio::test]
    async fn test_table_combined_modifiers() {
        let html = rsx! {
            <Table zebra=true pin_rows=true size=Size::Sm class="my-table">
                <tbody><tr><td>"A"</td></tr></tbody>
            </Table>
        };
        assert!(html.contains("table-zebra"));
        assert!(html.contains("table-pin-rows"));
        assert!(html.contains("table-sm"));
        assert!(html.contains("my-table"));
    }
}
