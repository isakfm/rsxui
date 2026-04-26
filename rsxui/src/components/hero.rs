//! Hero Component
//!
//! Hero is a component for displaying a large box or image with a title and description.
//!
//! # DaisyUI Classes
//! - Base: `hero`
//! - Part: `hero-content`, `hero-overlay`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{Hero, HeroContent};
//! use rsx::rsx;
//!
//! let html = rsx! {
//!     <Hero class="bg-base-200">
//!         <HeroContent>
//!             <h1 class="text-5xl font-bold">"Hello there"</h1>
//!             <p class="py-6">"Welcome to our site."</p>
//!         </HeroContent>
//!     </Hero>
//! };
//! ```

use rsx::{classes, component, rsx};

// ============================================
// HeroContent - Content wrapper inside hero
// ============================================

#[component]
pub fn HeroContent(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div class={classes!("hero-content", class)}>
            {children}
        </div>
    }
}

// ============================================
// HeroOverlay - Overlay for background image
// ============================================

#[component]
pub fn HeroOverlay(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div class={classes!("hero-overlay", class)}>
            {children}
        </div>
    }
}

// ============================================
// Hero - Hero section container
// ============================================

#[component]
pub fn Hero(
    #[builder(default)] class: String,
    #[builder(into)] style: Option<String>,
    children: String,
) -> String {
    rsx! {
        <div class={classes!("hero", class)} {super::attr_if("style", &style)}>
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
    use rsx::rsx;

    #[tokio::test]
    async fn test_hero_basic() {
        let html = rsx! {
            <Hero>
                <HeroContent>
                    <h1>"Title"</h1>
                </HeroContent>
            </Hero>
        };
        assert!(html.contains(r#"class="hero""#));
        assert!(html.contains(r#"class="hero-content""#));
        assert!(html.contains(">Title</h1>"));
    }

    #[tokio::test]
    async fn test_hero_with_overlay() {
        let html = rsx! {
            <Hero class="min-h-screen">
                <HeroOverlay class="bg-opacity-60" />
                <HeroContent>
                    <p>"Content"</p>
                </HeroContent>
            </Hero>
        };
        assert!(html.contains("min-h-screen"));
        assert!(html.contains(r#"class="hero-overlay bg-opacity-60""#));
        assert!(html.contains(">Content</p>"));
    }

    #[tokio::test]
    async fn test_hero_custom_classes() {
        let html = rsx! {
            <Hero class="bg-base-200">
                <HeroContent class="text-center">
                    "X"
                </HeroContent>
            </Hero>
        };
        assert!(html.contains("bg-base-200"));
        assert!(html.contains("text-center"));
    }
}
