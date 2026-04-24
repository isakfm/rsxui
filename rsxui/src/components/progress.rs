//! Progress Component
//!
//! Progress bar can be used to show the progress of a task or to show the passing of time.
//!
//! # DaisyUI Classes
//! - Base: `progress`
//! - Colors: `progress-neutral`, `progress-primary`, `progress-secondary`, `progress-accent`, `progress-info`, `progress-success`, `progress-warning`, `progress-error`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{Progress, Color};
//! use rsx_macros::rsx;
//!
//! let html = rsx! {
//!     <Progress value="70" max="100" color=Color::Primary />
//! };
//! ```

use rsx_macros::{classes, component, rsx};

use super::Color;

// ============================================
// Progress - Progress bar
// ============================================

#[component]
pub fn Progress(
    #[builder(into)] value: Option<String>,
    #[builder(into)] max: Option<String>,
    #[builder(default)] color: Color,
    #[builder(default)] class: String,
) -> String {
    rsx! {
        <progress
            class={classes!("progress", color.prefix("progress"), class)}
            {super::attr_if("value", &value)}
            {super::attr_if("max", &max)}
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
    async fn test_progress_basic() {
        let html = rsx! {
            <Progress value="50" max="100" />
        };
        assert!(html.contains(r#"class="progress""#));
        assert!(html.contains(r#"value="50""#));
        assert!(html.contains(r#"max="100""#));
    }

    #[tokio::test]
    async fn test_progress_indeterminate() {
        let html = rsx! {
            <Progress />
        };
        assert!(html.contains(r#"class="progress""#));
        assert!(!html.contains("value="));
        assert!(!html.contains("max="));
    }

    #[tokio::test]
    async fn test_progress_all_colors() {
        for color in [
            Color::Primary,
            Color::Secondary,
            Color::Accent,
            Color::Neutral,
            Color::Info,
            Color::Success,
            Color::Warning,
            Color::Error,
        ] {
            let html = rsx! {
                <Progress color=color />
            };
            let expected = color.prefix("progress");
            assert!(
                html.contains(&expected),
                "Missing color class {} for {:?}",
                expected,
                color
            );
        }
    }

    #[tokio::test]
    async fn test_progress_custom_class() {
        let html = rsx! {
            <Progress class="w-56" />
        };
        assert!(html.contains("w-56"));
    }
}
