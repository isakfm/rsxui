//! Carousel Component
//!
//! Carousel shows images or content in a scrollable area.
//!
//! # DaisyUI Classes
//! - Base: `carousel`
//! - Part: `carousel-item`
//! - Snap: `carousel-start`, `carousel-center`, `carousel-end`
//! - Direction: `carousel-horizontal`, `carousel-vertical`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{Carousel, CarouselItem, CarouselSnap};
//! use rsx::rsx;
//!
//! let html = rsx! {
//!     <Carousel snap={CarouselSnap::Center}>
//!         <CarouselItem><img src="1.jpg" /></CarouselItem>
//!         <CarouselItem><img src="2.jpg" /></CarouselItem>
//!     </Carousel>
//! };
//! ```

use enum_stringify::EnumStringify;
use rsx::{classes, component, rsx};

// ============================================
// CarouselSnap - Snap alignment for carousel
// ============================================

#[derive(Debug, Clone, PartialEq, Default, EnumStringify)]
#[enum_stringify(case = "kebab", prefix = "carousel-")]
pub enum CarouselSnap {
    #[default]
    #[enum_stringify(rename = "")]
    Default,
    Start,
    Center,
    End,
}

// ============================================
// CarouselDirection - Layout direction
// ============================================

#[derive(Debug, Clone, PartialEq, Default, EnumStringify)]
#[enum_stringify(case = "kebab", prefix = "carousel-")]
pub enum CarouselDirection {
    #[default]
    #[enum_stringify(rename = "")]
    Default,
    Horizontal,
    Vertical,
}

// ============================================
// CarouselItem - Individual carousel slide
// ============================================

#[component]
pub fn CarouselItem(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div class={classes!("carousel-item", class)}>{children}</div>
    }
}

// ============================================
// Carousel - Carousel container
// ============================================

#[component]
pub fn Carousel(
    #[builder(default)] snap: CarouselSnap,
    #[builder(default)] direction: CarouselDirection,
    #[builder(default)] class: String,
    #[builder(into)] aria_label: Option<String>,
    children: String,
) -> String {
    let aria = aria_label
        .map(|l| format!(r#" aria-label="{}""#, l))
        .unwrap_or_default();
    rsx! {
        <div
            tabindex="0"
            role="region"
            class={classes!("carousel", snap, direction, class)}
            {aria}
        >
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
    async fn test_carousel_basic() {
        let html = rsx! {
            <Carousel>
                <CarouselItem>"A"</CarouselItem>
            </Carousel>
        };
        assert!(html.contains(r#"class="carousel""#));
        assert!(html.contains("carousel-item"));
        assert!(html.contains(">A</div>"));
        assert!(html.contains(r#"tabindex="0""#));
        assert!(html.contains(r#"role="region""#));
    }

    #[tokio::test]
    async fn test_carousel_snaps() {
        for snap in [CarouselSnap::Start, CarouselSnap::Center, CarouselSnap::End] {
            let html = rsx! {
                <Carousel snap=snap.clone()>
                    <CarouselItem>"X"</CarouselItem>
                </Carousel>
            };
            let expected = snap.to_string();
            assert!(
                html.contains(&expected),
                "Missing snap class {} for {:?}",
                expected,
                snap
            );
        }
    }

    #[tokio::test]
    async fn test_carousel_directions() {
        for direction in [CarouselDirection::Horizontal, CarouselDirection::Vertical] {
            let html = rsx! {
                <Carousel direction=direction.clone()>
                    <CarouselItem>"X"</CarouselItem>
                </Carousel>
            };
            let expected = direction.to_string();
            assert!(
                html.contains(&expected),
                "Missing direction class {} for {:?}",
                expected,
                direction
            );
        }
    }

    #[tokio::test]
    async fn test_carousel_custom_class() {
        let html = rsx! {
            <Carousel class="rounded-box w-64">
                <CarouselItem class="w-full">"X"</CarouselItem>
            </Carousel>
        };
        assert!(html.contains("carousel rounded-box w-64"));
        assert!(html.contains("carousel-item w-full"));
    }

    #[tokio::test]
    async fn test_carousel_default_no_extra_classes() {
        let html = rsx! {
            <Carousel>
                <CarouselItem>"X"</CarouselItem>
            </Carousel>
        };
        assert!(!html.contains("carousel-horizontal"));
        assert!(!html.contains("carousel-start"));
    }

    #[tokio::test]
    async fn test_carousel_combined() {
        let html = rsx! {
            <Carousel snap=CarouselSnap::Center direction=CarouselDirection::Vertical class="h-96">
                <CarouselItem class="h-full">"A"</CarouselItem>
                <CarouselItem class="h-full">"B"</CarouselItem>
            </Carousel>
        };
        assert!(html.contains("carousel carousel-center carousel-vertical h-96"));
        assert!(html.contains("tabindex=\"0\""));
        assert!(html.contains("role=\"region\""));
    }

    #[tokio::test]
    async fn test_carousel_aria_label() {
        let html = rsx! {
            <Carousel aria_label="Image gallery">
                <CarouselItem>"A"</CarouselItem>
            </Carousel>
        };
        assert!(html.contains(r#"aria-label="Image gallery""#));
    }
}
