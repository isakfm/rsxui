//! Stack Component
//!
//! Stack visually puts elements on top of each other.
//!
//! # DaisyUI Classes
//! - Base: `stack`
//! - Modifier: `stack-top`, `stack-bottom`, `stack-start`, `stack-end`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{Stack, StackModifier};
//! use rsx::rsx;
//!
//! let html = rsx! {
//!     <Stack modifier={StackModifier::Top}>
//!         <img src="a.jpg" class="w-32" />
//!         <img src="b.jpg" class="w-32" />
//!         <img src="c.jpg" class="w-32" />
//!     </Stack>
//! };
//! ```

use enum_stringify::EnumStringify;
use rsx::{classes, component, rsx};

// ============================================
// StackModifier - Stack alignment modifier
// ============================================

#[derive(Debug, Clone, PartialEq, Default, EnumStringify)]
#[enum_stringify(case = "kebab", prefix = "stack-")]
pub enum StackModifier {
    #[default]
    #[enum_stringify(rename = "")]
    Default,
    Top,
    Bottom,
    Start,
    End,
}

// ============================================
// Stack - Overlapping elements container
// ============================================

#[component]
pub fn Stack(
    #[builder(default)] modifier: StackModifier,
    #[builder(default)] class: String,
    children: String,
) -> String {
    rsx! {
        <div class={classes!("stack", modifier.to_string(), class)}>
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
    async fn test_stack_basic() {
        let html = rsx! {
            <Stack>
                <div>"A"</div>
                <div>"B"</div>
            </Stack>
        };
        assert!(html.contains(r#"class="stack""#));
        assert!(html.contains(">A</div>"));
        assert!(html.contains(">B</div>"));
    }

    #[tokio::test]
    async fn test_stack_modifiers() {
        for modifier in [
            StackModifier::Top,
            StackModifier::Bottom,
            StackModifier::Start,
            StackModifier::End,
        ] {
            let html = rsx! {
                <Stack modifier=modifier.clone()>"X"</Stack>
            };
            let expected = modifier.to_string();
            assert!(
                html.contains(&expected),
                "Missing modifier class {} for {:?}",
                expected,
                modifier
            );
        }
    }

    #[tokio::test]
    async fn test_stack_custom_class() {
        let html = rsx! {
            <Stack class="w-32">"X"</Stack>
        };
        assert!(html.contains("stack w-32"));
    }

    #[tokio::test]
    async fn test_stack_default_no_extra_classes() {
        let html = rsx! {
            <Stack>"X"</Stack>
        };
        assert!(!html.contains("stack-top"));
        assert!(!html.contains("stack-bottom"));
    }
}
