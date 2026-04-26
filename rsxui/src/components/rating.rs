//! Rating Component
//!
//! A rating component with DaisyUI styling.
//!
//! # DaisyUI Classes
//! - Base: `rating`
//! - Sizes: `rating-xs`, `rating-sm`, `rating-md`, `rating-lg`, `rating-xl`
//! - Half: `rating-half`
//! - Hidden: `rating-hidden` (on first input to allow clearing)
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{Rating, Size};
//! use rsx::rsx;
//!
//! let html = rsx! {
//!     <Rating max=5 size=Size::Md />
//! };
//! ```

use rsx::{classes, component, rsx};

use super::{class_if, show_if, Size};

// ============================================
// Rating - DaisyUI rating
// ============================================

#[component]
pub fn Rating(
    #[builder(default = 5)] max: u8,
    #[builder(default)] size: Size,
    #[builder(default)] half: bool,
    #[builder(default)] hidden: bool,
    #[builder(default)] name: String,
    #[builder(default)] mask: String,
    #[builder(default)] color: String,
    #[builder(default)] checked: u8,
    #[builder(default)] read_only: bool,
    #[builder(default)] class: String,
) -> String {
    let name = if name.is_empty() {
        "rating".to_string()
    } else {
        name
    };

    let mask_class = if mask.is_empty() {
        "mask-star".to_string()
    } else {
        mask
    };

    let item_count = if half { max as u16 * 2 } else { max as u16 };

    let mut items = Vec::new();

    // Hidden clear input (optional)
    if hidden {
        let is_checked = checked == 0;
        if read_only {
            items.push(rsx! {
                <div class="rating-hidden" aria-label="clear" />
            });
        } else {
            items.push(rsx! {
                <input
                    type="radio"
                    name={name.clone()}
                    class="rating-hidden"
                    aria-label="clear"
                    {show_if(is_checked, "checked")}
                />
            });
        }
    }

    for i in 1..=item_count {
        let star_num = if half { (i as f32) / 2.0 } else { i as f32 };

        let aria_label = if half && i % 2 == 1 {
            format!("{} star", star_num)
        } else if half {
            format!("{} star", star_num)
        } else {
            format!("{} star", i)
        };

        let is_checked = u16::from(checked) == i;

        let half_class = if half {
            if i % 2 == 1 {
                "mask-half-1"
            } else {
                "mask-half-2"
            }
        } else {
            ""
        };

        let item_class = if color.is_empty() {
            format!("mask {} {}", mask_class, half_class)
        } else {
            format!("mask {} {} {}", mask_class, half_class, color)
        }
        .trim()
        .to_string();

        if read_only {
            let current = if is_checked {
                r#" aria-current="true""#
            } else {
                ""
            };
            items.push(rsx! {
                <div
                class={item_class}
                aria-label={aria_label} {current}></div>
            });
        } else {
            items.push(rsx! {
                <input
                    type="radio"
                    name={name.clone()}
                    class={item_class}
                    aria-label={aria_label}
                    {show_if(is_checked, "checked")}
                />
            });
        }
    }

    let items_html = items.join("");

    rsx! {
        <div class={classes!(
            "rating",
            size.prefix("rating"),
            class_if(half, "rating-half"),
            class,
        )}>
            {items_html}
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
    async fn test_rating_basic() {
        let html = rsx! {
            <Rating />
        };
        assert!(html.contains(r#"<div class="rating""#));
        assert!(html.contains(r#"name="rating""#));
        assert!(html.contains(r#"aria-label="1 star""#));
        assert!(html.contains(r#"aria-label="5 star""#));
    }

    #[tokio::test]
    async fn test_rating_custom_max() {
        let html = rsx! {
            <Rating max=3 />
        };
        assert!(html.contains(r#"aria-label="1 star""#));
        assert!(html.contains(r#"aria-label="2 star""#));
        assert!(html.contains(r#"aria-label="3 star""#));
        assert!(!html.contains(r#"aria-label="4 star""#));
    }

    #[tokio::test]
    async fn test_rating_with_size() {
        for size in [Size::Xs, Size::Sm, Size::Md, Size::Lg, Size::Xl] {
            let html = rsx! {
                <Rating size=size />
            };
            let expected = size.prefix("rating");
            assert!(
                html.contains(&expected),
                "Missing size class {} for {:?}",
                expected,
                size
            );
        }
    }

    #[tokio::test]
    async fn test_rating_half() {
        let html = rsx! {
            <Rating half=true max=3 />
        };
        assert!(html.contains("rating-half"));
        assert!(html.contains("mask-half-1"));
        assert!(html.contains("mask-half-2"));
        // 3 stars * 2 halves = 6 inputs
        let input_count = html.matches("<input").count();
        assert_eq!(input_count, 6);
    }

    #[tokio::test]
    async fn test_rating_half_labels() {
        let html = rsx! {
            <Rating half=true max=2 />
        };
        assert!(html.contains(r#"aria-label="1 star""#));
        assert!(html.contains(r#"aria-label="1.5 star""#));
        assert!(html.contains(r#"aria-label="2 star""#));
    }

    #[tokio::test]
    async fn test_rating_hidden() {
        let html = rsx! {
            <Rating hidden=true max=3 />
        };
        // rating-hidden class on the first input, not container
        assert!(html.contains(r#"class="rating-hidden""#));
        assert!(html.contains(r#"aria-label="clear""#));
        // 3 stars + 1 hidden = 4 inputs
        let input_count = html.matches("<input").count();
        assert_eq!(input_count, 4);
    }

    #[tokio::test]
    async fn test_rating_hidden_and_half() {
        let html = rsx! {
            <Rating hidden=true half=true max=2 />
        };
        // 1 hidden + 4 half inputs = 5 inputs
        let input_count = html.matches("<input").count();
        assert_eq!(input_count, 5);
        assert!(html.contains(r#"aria-label="clear""#));
        assert!(html.contains("mask-half-1"));
    }

    #[tokio::test]
    async fn test_rating_custom_mask() {
        let html = rsx! {
            <Rating mask="mask-heart" />
        };
        assert!(html.contains("mask-heart"));
    }

    #[tokio::test]
    async fn test_rating_custom_color() {
        let html = rsx! {
            <Rating color="bg-orange-400" />
        };
        assert!(html.contains("bg-orange-400"));
    }

    #[tokio::test]
    async fn test_rating_checked() {
        let html = rsx! {
            <Rating checked=3 />
        };
        // Only one input should have checked attribute
        let checked_count = html.matches(" checked").count();
        assert_eq!(checked_count, 1);
        assert!(html.contains(" checked"));
    }

    #[tokio::test]
    async fn test_rating_checked_with_hidden() {
        let html = rsx! {
            <Rating hidden=true checked=2 />
        };
        // checked=2 means the 2nd visible input (3rd overall) is checked
        let checked_count = html.matches(" checked").count();
        assert_eq!(checked_count, 1);
    }

    #[tokio::test]
    async fn test_rating_checked_with_half() {
        let html = rsx! {
            <Rating half=true checked=3 />
        };
        // checked=3 means 1.5 stars selected
        let checked_count = html.matches(" checked").count();
        assert_eq!(checked_count, 1);
    }

    #[tokio::test]
    async fn test_rating_read_only() {
        let html = rsx! {
            <Rating read_only=true checked=3 />
        };
        // Should render divs, not inputs
        assert!(!html.contains(r#"type="radio""#));
        assert!(html.contains("aria-current=\"true\""));
        assert!(html.contains("<div"));
    }

    #[tokio::test]
    async fn test_rating_read_only_no_checked() {
        let html = rsx! {
            <Rating read_only=true />
        };
        assert!(!html.contains("aria-current"));
    }

    #[tokio::test]
    async fn test_rating_custom_name() {
        let html = rsx! {
            <Rating name="product-rating".to_string() />
        };
        assert!(html.contains(r#"name="product-rating""#));
    }

    #[tokio::test]
    async fn test_rating_custom_class() {
        let html = rsx! {
            <Rating class="my-rating".to_string() />
        };
        assert!(html.contains(r#"class="rating my-rating""#));
    }

    #[tokio::test]
    async fn test_rating_combined_modifiers() {
        let html = rsx! {
            <Rating max=4 size=Size::Lg half=true hidden=true mask="mask-star-2" color="bg-green-500" checked=3 class="fancy".to_string() />
        };
        assert!(html.contains("rating-lg"));
        assert!(html.contains("rating-half"));
        assert!(html.contains("mask-star-2"));
        assert!(html.contains("bg-green-500"));
        assert!(html.contains("fancy"));
        assert!(html.contains(r#"aria-label="clear""#));
        assert!(html.contains("mask-half-1"));
        assert!(html.contains("mask-half-2"));
    }
}
