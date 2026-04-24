//! Radial Progress Component
//!
//! Radial progress can be used to show the progress of a task or to show the passing of time.
//!
//! # DaisyUI Classes
//! - Base: `radial-progress`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::RadialProgress;
//! use rsx_macros::rsx;
//!
//! let html = rsx! {
//!     <RadialProgress value="70">"70%"</RadialProgress>
//! };
//! ```

use rsx_macros::{classes, component, rsx};

// ============================================
// RadialProgress - Circular progress indicator
// ============================================

#[component]
pub fn RadialProgress(
    value: String,
    #[builder(into)] size: Option<String>,
    #[builder(into)] thickness: Option<String>,
    #[builder(default)] class: String,
    children: String,
) -> String {
    let style_value = format!("--value:{};", value);
    let size_style = size.map(|s| format!("--size:{};", s)).unwrap_or_default();
    let thickness_style = thickness
        .map(|t| format!("--thickness:{};", t))
        .unwrap_or_default();
    let full_style = format!("{}{}{}", style_value, size_style, thickness_style);

    rsx! {
        <div
            class={classes!("radial-progress", class)}
            style={full_style}
            aria-valuenow={value}
            role="progressbar">
            {children}
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
    async fn test_radial_progress_basic() {
        let html = rsx! {
            <RadialProgress value="70">"70%"</RadialProgress>
        };
        assert!(html.contains(r#"class="radial-progress""#));
        assert!(html.contains(r#"style="--value:70;""#));
        assert!(html.contains(r#"aria-valuenow="70""#));
        assert!(html.contains(r#"role="progressbar""#));
        assert!(html.contains(">70%</div>"));
    }

    #[tokio::test]
    async fn test_radial_progress_with_size() {
        let html = rsx! {
            <RadialProgress value="50" size="4rem">"50%"</RadialProgress>
        };
        assert!(html.contains("--size:4rem;"));
    }

    #[tokio::test]
    async fn test_radial_progress_with_thickness() {
        let html = rsx! {
            <RadialProgress value="30" thickness="4px">"30%"</RadialProgress>
        };
        assert!(html.contains("--thickness:4px;"));
    }

    #[tokio::test]
    async fn test_radial_progress_combined_styles() {
        let html = rsx! {
            <RadialProgress value="90" size="6rem" thickness="8px">"90%"</RadialProgress>
        };
        assert!(html.contains("--value:90;"));
        assert!(html.contains("--size:6rem;"));
        assert!(html.contains("--thickness:8px;"));
    }

    #[tokio::test]
    async fn test_radial_progress_custom_class() {
        let html = rsx! {
            <RadialProgress value="10" class="text-primary">"10%"</RadialProgress>
        };
        assert!(html.contains("radial-progress text-primary"));
    }
}
