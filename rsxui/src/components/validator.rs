//! Validator Component
//!
//! A wrapper that places a validation hint after a form element.
//!
//! In DaisyUI, the `validator` class is applied directly to the input/select/textarea,
//! and the `validator-hint` element is a sibling that appears after it.
//!
//! # DaisyUI Classes
//! - `validator` — Applied to the input/select/textarea itself
//! - `validator-hint` — Hint text shown when the preceding validator element is invalid
//! - `validator-hint hidden` — Hint that takes no space when invisible (no layout shift)
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::Validator;
//! use rsx_macros::rsx;
//!
//! let html = rsx! {
//!     <Validator hint="Enter valid email address">
//!         <input type="email" class="input validator" required placeholder="mail@site.com" />
//!     </Validator>
//! };
//! ```

use rsx_macros::{classes, component, rsx};

use super::class_if;

// ============================================
// ValidatorHint — Standalone hint element
// ============================================

#[component]
pub fn ValidatorHint(
    #[builder(into)] hint: String,
    #[builder(default)] hidden: bool,
    #[builder(default)] class: String,
) -> String {
    rsx! {
        <div class={classes!("validator-hint", class_if(hidden, "hidden"), class)}>
            {hint}
        </div>
    }
}

// ============================================
// Validator — Wrapper: children + hint
// ============================================

#[component]
pub fn Validator(
    #[builder(into)] hint: String,
    #[builder(default)] hidden_hint: bool,
    #[builder(default)] class: String,
    children: String,
) -> String {
    rsx! {
        <div class={class}>
            {children}
            <ValidatorHint hint=hint hidden=hidden_hint />
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
    async fn test_validator_basic() {
        let html = rsx! {
            <Validator hint="Required field">
                <input type="email" class="input validator" required />
            </Validator>
        };
        // Children (input) comes before hint
        let input_pos = html.find(r#"<input"#).unwrap();
        let hint_pos = html.find(r#"validator-hint"#).unwrap();
        assert!(input_pos < hint_pos, "Input should come before hint");
        assert!(html.contains(r#"class="validator-hint""#));
        assert!(html.contains(">Required field</div>"));
        assert!(html.contains(r#"class="input validator""#));
    }

    #[tokio::test]
    async fn test_validator_custom_class() {
        let html = rsx! {
            <Validator hint="Hint" class="w-full max-w-xs">
                <input type="text" class="input validator" required />
            </Validator>
        };
        assert!(html.contains(r#"class="w-full max-w-xs""#));
    }

    #[tokio::test]
    async fn test_validator_hidden_hint() {
        let html = rsx! {
            <Validator hint="Required" hidden_hint=true>
                <input type="text" class="input validator" required />
            </Validator>
        };
        assert!(html.contains(r#"class="validator-hint hidden""#));
    }

    #[tokio::test]
    async fn test_validator_hint_standalone() {
        let html = rsx! {
            <ValidatorHint hint="Must be 10 digits" />
        };
        assert!(html.contains(r#"class="validator-hint""#));
        assert!(html.contains(">Must be 10 digits</div>"));
    }

    #[tokio::test]
    async fn test_validator_hint_hidden() {
        let html = rsx! {
            <ValidatorHint hint="Required" hidden=true />
        };
        assert!(html.contains(r#"class="validator-hint hidden""#));
    }
}
