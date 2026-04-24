//! Indicator Component
//!
//! Indicators are used to place an element on the corner of another element.
//!
//! # DaisyUI Classes
//! - Base: `indicator`
//! - Part: `indicator-item`
//! - Placement: `indicator-start`, `indicator-center`, `indicator-end`, `indicator-top`, `indicator-middle`, `indicator-bottom`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{Indicator, IndicatorItem};
//! use rsx_macros::rsx;
//!
//! let html = rsx! {
//!     <Indicator>
//!         <IndicatorItem>
//!             <span class="badge badge-primary">"New"</span>
//!         </IndicatorItem>
//!         <div class="bg-base-300 grid h-32 w-32 place-items-center">"Content"</div>
//!     </Indicator>
//! };
//! ```

use rsx_macros::{classes, component, rsx};

use super::class_if;

// ============================================
// IndicatorItem - Badge/element on corner
// ============================================

#[component]
pub fn IndicatorItem(
    #[builder(default)] start: bool,
    #[builder(default)] center: bool,
    #[builder(default)] end_: bool,
    #[builder(default)] top: bool,
    #[builder(default)] middle: bool,
    #[builder(default)] bottom: bool,
    #[builder(default)] class: String,
    children: String,
) -> String {
    rsx! {
        <span class={classes!(
            "indicator-item",
            class_if(start, "indicator-start"),
            class_if(center, "indicator-center"),
            class_if(end_, "indicator-end"),
            class_if(top, "indicator-top"),
            class_if(middle, "indicator-middle"),
            class_if(bottom, "indicator-bottom"),
            class,
        )}>
            {children}
        </span>
    }
}

// ============================================
// Indicator - Container with corner element
// ============================================

#[component]
pub fn Indicator(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div class={classes!("indicator", class)}>
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
    async fn test_indicator_basic() {
        let html = rsx! {
            <Indicator>
                <IndicatorItem>
                    <span class="badge">"New"</span>
                </IndicatorItem>
                <div>"Content"</div>
            </Indicator>
        };
        assert!(html.contains(r#"class="indicator""#));
        assert!(html.contains(r#"class="indicator-item""#));
        assert!(html.contains(">New</span>"));
        assert!(html.contains(">Content</div>"));
    }

    #[tokio::test]
    async fn test_indicator_placements() {
        let html = rsx! {
            <IndicatorItem start=true center=true end_=true top=true middle=true bottom=true>
                "X"
            </IndicatorItem>
        };
        assert!(html.contains("indicator-start"));
        assert!(html.contains("indicator-center"));
        assert!(html.contains("indicator-end"));
        assert!(html.contains("indicator-top"));
        assert!(html.contains("indicator-middle"));
        assert!(html.contains("indicator-bottom"));
    }

    #[tokio::test]
    async fn test_indicator_custom_class() {
        let html = rsx! {
            <Indicator class="inline-block">
                <IndicatorItem class="badge-primary">"X"</IndicatorItem>
            </Indicator>
        };
        assert!(html.contains("inline-block"));
        assert!(html.contains("badge-primary"));
    }
}
