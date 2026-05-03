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
//! use rsx::rsx;
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

use rsx::attrs::RenderAttrs;
use rsx::{classes, rsx, ui};

use super::class_if;

// ============================================
// IndicatorItem - Badge/element on corner
// ============================================

#[ui]
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
            class_if(props.start, "indicator-start"),
            class_if(props.center, "indicator-center"),
            class_if(props.end_, "indicator-end"),
            class_if(props.top, "indicator-top"),
            class_if(props.middle, "indicator-middle"),
            class_if(props.bottom, "indicator-bottom"),
            props.class,
        )} {props.render_attrs()}>
            {props.children}
        </span>
    }
}

// ============================================
// Indicator - Container with corner element
// ============================================

#[ui]
pub fn Indicator(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div class={classes!("indicator", props.class)} {props.render_attrs()}>
            {props.children}
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
