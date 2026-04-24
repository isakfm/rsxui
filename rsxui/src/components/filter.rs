//! Filter Component
//!
//! A filter component with DaisyUI styling.
//!
//! # DaisyUI Classes
//! - Base: `filter`
//! - Reset: `filter-reset`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::Filter;
//! use rsx_macros::rsx;
//!
//! let html = rsx! {
//!     <Filter tabs=vec!["Tab 1".to_string(), "Tab 2".to_string()] />
//! };
//! ```

use rsx_macros::{classes, component, rsx};

// ============================================
// Filter - DaisyUI filter
// ============================================

#[component]
pub fn Filter(
    tabs: Vec<String>,
    #[builder(default)] name: String,
    #[builder(default)] use_form: bool,
    #[builder(default)] reset_label: String,
    #[builder(default)] class: String,
) -> String {
    let name = if name.is_empty() {
        "filter-tabs".to_string()
    } else {
        name
    };
    let reset_label = if reset_label.is_empty() {
        "×".to_string()
    } else {
        reset_label
    };

    if use_form {
        rsx! {
            <form class={classes!("filter", class)}>
                <input class="btn btn-square" type="reset" value={reset_label} />
                {
                    for tab in &tabs {
                        rsx! {
                            <input class="btn" type="radio" name={name.clone()} aria-label={tab} />
                        }
                    }
                }
            </form>
        }
    } else {
        rsx! {
            <div class={classes!("filter", class)}>
                <input class="btn filter-reset" type="radio" name={name.clone()} aria-label={reset_label} />
                {
                    for tab in &tabs {
                        rsx! {
                            <input class="btn" type="radio" name={name.clone()} aria-label={tab} />
                        }
                    }
                }
            </div>
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
    async fn test_filter_basic() {
        let html = rsx! {
            <Filter tabs=vec!["Tab 1".to_string(), "Tab 2".to_string()] />
        };
        assert!(html.contains(r#"<div class="filter""#));
        assert!(html.contains(r#"class="btn filter-reset""#));
        assert!(html.contains(r#"aria-label="×""#));
        assert!(html.contains(r#"aria-label="Tab 1""#));
        assert!(html.contains(r#"aria-label="Tab 2""#));
        assert!(html.contains(r#"name="filter-tabs""#));
    }

    #[tokio::test]
    async fn test_filter_with_form() {
        let html = rsx! {
            <Filter tabs=vec!["All".to_string(), "Active".to_string()] use_form=true />
        };
        assert!(html.contains(r#"<form class="filter""#));
        assert!(html.contains(r#"type="reset""#));
        assert!(html.contains(r#"value="×""#));
        assert!(!html.contains(r#"filter-reset"#));
    }

    #[tokio::test]
    async fn test_filter_custom_name() {
        let html = rsx! {
            <Filter tabs=vec!["A".to_string()] name="my-filter".to_string() />
        };
        assert!(html.contains(r#"name="my-filter""#));
    }

    #[tokio::test]
    async fn test_filter_custom_reset_label() {
        let html = rsx! {
            <Filter tabs=vec!["A".to_string()] reset_label="Clear".to_string() />
        };
        assert!(html.contains(r#"aria-label="Clear""#));
    }

    #[tokio::test]
    async fn test_filter_custom_class() {
        let html = rsx! {
            <Filter tabs=vec!["A".to_string()] class="my-filter".to_string() />
        };
        assert!(html.contains(r#"class="filter my-filter""#));
    }

    #[tokio::test]
    async fn test_filter_empty_tabs() {
        let html = rsx! {
            <Filter tabs=vec![] />
        };
        assert!(html.contains(r#"<div class="filter""#));
        assert!(html.contains(r#"class="btn filter-reset""#));
    }
}
