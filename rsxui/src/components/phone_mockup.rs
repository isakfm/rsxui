//! Phone Mockup Component
//!
//! Phone mockup shows a mockup of an iPhone.
//!
//! # DaisyUI Classes
//! - Base: `mockup-phone`
//! - Part: `mockup-phone-camera`, `mockup-phone-display`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::PhoneMockup;
//! use rsx::rsx;
//!
//! let html = rsx! {
//!     <PhoneMockup>
//!         <div class="artboard artboard-demo phone-1">"Hi."</div>
//!     </PhoneMockup>
//! };
//! ```

use rsx::attrs::RenderAttrs;
use rsx::{classes, rsx, ui};

// ============================================
// PhoneMockup - iPhone mockup
// ============================================

#[ui]
pub fn PhoneMockup(
    #[builder(default)] class: String,
    inner_class: String,
    children: String,
) -> String {
    rsx! {
        <div class={classes!("mockup-phone", props.class)} {props.render_attrs()}>
            <div class="mockup-phone-camera" />
            <div class={classes!("mockup-phone-display", props.inner_class)}>{props.children}</div>
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
    async fn test_phone_mockup_basic() {
        let html = rsx! {
            <PhoneMockup>
                <div>"Hi."</div>
            </PhoneMockup>
        };
        assert!(html.contains(r#"class="mockup-phone""#));
        assert!(html.contains(r#"class="mockup-phone-camera""#));
        assert!(html.contains(r#"class="mockup-phone-display""#));
        assert!(html.contains(">Hi.</div>"));
    }

    #[tokio::test]
    async fn test_phone_mockup_custom_class() {
        let html = rsx! {
            <PhoneMockup class="border-primary">"X"</PhoneMockup>
        };
        assert!(html.contains("border-primary"));
    }
}
