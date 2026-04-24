//! Loading Component
//!
//! Loading shows an animation to indicate that something is loading.
//!
//! # DaisyUI Classes
//! - Base: `loading`
//! - Styles: `loading-spinner`, `loading-dots`, `loading-ring`, `loading-ball`, `loading-bars`, `loading-infinity`
//! - Sizes: `loading-xs`, `loading-sm`, `loading-md`, `loading-lg`, `loading-xl`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{Loading, LoadingStyle, Size};
//! use rsx_macros::rsx;
//!
//! let html = rsx! {
//!     <Loading style=LoadingStyle::Spinner size=Size::Lg />
//! };
//! ```

use enum_stringify::EnumStringify;
use rsx_macros::{classes, component, rsx};

use super::Size;

// ============================================
// LoadingStyle - Loading animation style
// ============================================

#[derive(Debug, Clone, Copy, PartialEq, Default, EnumStringify)]
#[enum_stringify(case = "kebab", prefix = "loading-")]
pub enum LoadingStyle {
    #[default]
    Spinner,
    Dots,
    Ring,
    Ball,
    Bars,
    Infinity,
}

// ============================================
// Loading - Loading animation
// ============================================

#[component]
pub fn Loading(
    #[builder(default)] style: LoadingStyle,
    #[builder(default)] size: Size,
    #[builder(default)] class: String,
) -> String {
    rsx! {
        <span class={classes!(
            "loading",
            style,
            size.prefix("loading"),
            class,
        )} />
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
    async fn test_loading_basic() {
        let html = rsx! {
            <Loading />
        };
        assert!(html.contains("loading"));
    }

    #[tokio::test]
    async fn test_loading_all_styles() {
        for style in [
            LoadingStyle::Spinner,
            LoadingStyle::Dots,
            LoadingStyle::Ring,
            LoadingStyle::Ball,
            LoadingStyle::Bars,
            LoadingStyle::Infinity,
        ] {
            let html = rsx! {
                <Loading style=style />
            };
            let expected = style.to_string();
            assert!(
                html.contains(&expected),
                "Missing style class {} for {:?}",
                expected,
                style
            );
        }
    }

    #[tokio::test]
    async fn test_loading_all_sizes() {
        for size in [Size::Xs, Size::Sm, Size::Md, Size::Lg, Size::Xl] {
            let html = rsx! {
                <Loading size=size />
            };
            let expected = size.prefix("loading");
            assert!(
                html.contains(&expected),
                "Missing size class {} for {:?}",
                expected,
                size
            );
        }
    }

    #[tokio::test]
    async fn test_loading_custom_class() {
        let html = rsx! {
            <Loading class="text-primary" />
        };
        assert!(html.contains("text-primary"));
    }
}
