//! Hover Gallery Component
//!
//! Hover Gallery is a container of images. The first image is visible by default
//! and when we hover it horizontally, other images show up.
//!
//! # DaisyUI Classes
//! - Base: `hover-gallery`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::HoverGallery;
//! use rsx::rsx;
//!
//! let html = rsx! {
//!     <HoverGallery class="max-w-60">
//!         <img src="hat-1.jpg" />
//!         <img src="hat-2.jpg" />
//!         <img src="hat-3.jpg" />
//!     </HoverGallery>
//! };
//! ```

use rsx::{classes, component, rsx};

// ============================================
// HoverGallery - Image hover gallery
// ============================================

#[component]
pub fn HoverGallery(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <figure class={classes!("hover-gallery", class)}>
            {children}
        </figure>
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
    async fn test_hover_gallery_basic() {
        let html = rsx! {
            <HoverGallery>
                <img src="1.jpg" />
                <img src="2.jpg" />
                <img src="3.jpg" />
            </HoverGallery>
        };
        assert!(html.contains(r#"class="hover-gallery""#));
        assert!(html.contains("<figure"));
        assert!(html.contains("</figure>"));
        assert!(html.contains(r#"src="1.jpg""#));
        assert!(html.contains(r#"src="2.jpg""#));
        assert!(html.contains(r#"src="3.jpg""#));
    }

    #[tokio::test]
    async fn test_hover_gallery_custom_class() {
        let html = rsx! {
            <HoverGallery class="max-w-60">
                <img src="1.jpg" />
            </HoverGallery>
        };
        assert!(html.contains("hover-gallery max-w-60"));
    }

    #[tokio::test]
    async fn test_hover_gallery_up_to_ten_images() {
        let html = rsx! {
            <HoverGallery>
                <img src="1.jpg" />
                <img src="2.jpg" />
                <img src="3.jpg" />
                <img src="4.jpg" />
                <img src="5.jpg" />
            </HoverGallery>
        };
        let img_count = html.matches("<img").count();
        assert_eq!(img_count, 5);
    }
}
