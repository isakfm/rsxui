//! Steps Component
//!
//! Steps can be used to show a list of steps in a process.
//!
//! # DaisyUI Classes
//! - Base: `steps`
//! - Part: `step`, `step-icon`
//! - Color: `step-neutral`, `step-primary`, `step-secondary`, `step-accent`, `step-info`, `step-success`, `step-warning`, `step-error`
//! - Direction: `steps-vertical`, `steps-horizontal`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{Steps, Step, Color, StepDirection};
//! use rsx_macros::rsx;
//!
//! let html = rsx! {
//!     <Steps>
//!         <Step color=Color::Primary>"Register"</Step>
//!         <Step>"Choose plan"</Step>
//!         <Step>"Purchase"</Step>
//!     </Steps>
//! };
//! ```

use enum_stringify::EnumStringify;
use rsx_macros::{classes, rsx, ui};

use super::{attr_if, class_if, Color};

// ============================================
// StepDirection - Steps layout direction
// ============================================

#[derive(Debug, Clone, PartialEq, Default, EnumStringify)]
#[enum_stringify(case = "kebab", prefix = "steps-")]
pub enum StepDirection {
    #[default]
    #[enum_stringify(rename = "")]
    Default,
    Vertical,
    Horizontal,
}

// ============================================
// Step - Individual step item
// ============================================

#[ui]
pub fn Step(
    #[builder(default)] color: Color,
    #[builder(into)] data_content: Option<String>,
    #[builder(default)] icon: bool,
    #[builder(default)] class: String,
    children: String,
) -> String {
    rsx! {
        <li
            class=classes!("step", props.color.prefix("step"), class_if(props.icon, "step-icon"), props.class)
            {attr_if("data-content", &props.data_content)}>
            {props.children}
        </li>
    }
}

// ============================================
// Steps - Steps container
// ============================================

#[ui]
pub fn Steps(
    #[builder(default)] direction: StepDirection,
    #[builder(default)] color: Color,
    #[builder(default)] class: String,
    children: String,
) -> String {
    rsx! {
        <ul class=classes!("steps", props.direction, props.color.prefix("steps"), props.class)>
            {props.children}
        </ul>
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
    async fn test_steps_basic() {
        let html = rsx! {
            <Steps>
                <Step>"Register"</Step>
                <Step>"Choose plan"</Step>
                <Step>"Purchase"</Step>
            </Steps>
        };
        assert!(html.contains(r#"class="steps""#));
        assert!(html.contains("step"));
        assert!(html.contains(">Register</li>"));
        assert!(html.contains(">Choose plan</li>"));
        assert!(html.contains(">Purchase</li>"));
    }

    #[tokio::test]
    async fn test_steps_direction() {
        let html = rsx! {
            <Steps direction=StepDirection::Vertical>
                <Step>"Step 1"</Step>
            </Steps>
        };
        assert!(html.contains("steps-vertical"));
    }

    #[tokio::test]
    async fn test_step_color() {
        let html = rsx! {
            <Step color=Color::Primary>"Active"</Step>
        };
        assert!(html.contains("step-primary"));
    }

    #[tokio::test]
    async fn test_steps_color() {
        let html = rsx! {
            <Steps color=Color::Secondary>
                <Step>"All secondary"</Step>
            </Steps>
        };
        assert!(html.contains("steps-secondary"));
    }

    #[tokio::test]
    async fn test_step_data_content() {
        let html = rsx! {
            <Step data_content="✓">"Done"</Step>
        };
        assert!(html.contains(r#"data-content="✓""#));
    }

    #[tokio::test]
    async fn test_step_icon() {
        let html = rsx! {
            <Step icon=true>"With icon"</Step>
        };
        assert!(html.contains("step-icon"));
    }

    #[tokio::test]
    async fn test_steps_custom_class() {
        let html = rsx! {
            <Steps class="w-full">
                <Step class="text-sm">"Small"</Step>
            </Steps>
        };
        assert!(html.contains("steps w-full"));
        assert!(html.contains("step text-sm"));
    }

    #[tokio::test]
    async fn test_step_default_no_extra_classes() {
        let html = rsx! {
            <Step>"Default"</Step>
        };
        assert!(!html.contains("step-primary"));
        assert!(!html.contains("step-icon"));
    }
}
