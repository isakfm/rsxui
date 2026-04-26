//! Calendar Component
//!
//! Calendar includes styles for Cally web component.
//! DaisyUI also supports Pikaday and React Day Picker via CSS classes.
//!
//! # DaisyUI Classes
//! - Cally: `cally`
//! - Pikaday input: `pika-single`
//! - React Day Picker: `react-day-picker`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::CallyCalendar;
//! use rsx::rsx;
//!
//! let html = rsx! {
//!     <CallyCalendar class="bg-base-100 border border-base-300 shadow-lg rounded-box">
//!         <svg aria-label="Previous" slot="previous">"..."</svg>
//!         <svg aria-label="Next" slot="next">"..."</svg>
//!         <calendar-month></calendar-month>
//!     </CallyCalendar>
//! };
//! ```

use rsx::{classes, component, rsx};

// ============================================
// CallyCalendar - Cally web component wrapper
// ============================================

#[component]
pub fn CallyCalendar(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <calendar-date class={classes!("cally", class)}>
            {children}
        </calendar-date>
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
    async fn test_cally_calendar_basic() {
        let html = rsx! {
            <CallyCalendar>
                <calendar-month></calendar-month>
            </CallyCalendar>
        };
        assert!(html.contains("<calendar-date"));
        assert!(html.contains(r#"class="cally""#));
        assert!(html.contains("<calendar-month>"));
    }

    #[tokio::test]
    async fn test_cally_calendar_custom_class() {
        let html = rsx! {
            <CallyCalendar class="bg-base-100 rounded-box">
                <calendar-month></calendar-month>
            </CallyCalendar>
        };
        assert!(html.contains("cally bg-base-100 rounded-box"));
    }
}
