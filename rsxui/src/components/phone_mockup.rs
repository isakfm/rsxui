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

use rsx::{classes, component, rsx};

// ============================================
// PhoneMockup - iPhone mockup
// ============================================

#[component]
pub fn PhoneMockup(
    #[builder(default)] class: String,
    inner_class: String,
    children: String,
) -> String {
    rsx! {
        <div class={classes!("mockup-phone", class)}>
            <div class="mockup-phone-camera" />
            <div class={classes!("mockup-phone-display", inner_class)}>{children}</div>
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
