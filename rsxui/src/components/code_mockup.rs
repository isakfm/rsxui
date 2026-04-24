//! Code Mockup Component
//!
//! Code mockup is used to show a block of code in a box that looks like a code editor.
//!
//! # DaisyUI Classes
//! - Base: `mockup-code`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::CodeMockup;
//! use rsx_macros::rsx;
//!
//! let html = rsx! {
//!     <CodeMockup>
//!         <pre data-prefix="$"><code>"npm i daisyui"</code></pre>
//!         <pre data-prefix=">" class="text-warning"><code>"installing..."</code></pre>
//!     </CodeMockup>
//! };
//! ```

use rsx_macros::{classes, component, rsx};

// ============================================
// CodeMockup - Code editor mockup
// ============================================

#[component]
pub fn CodeMockup(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div class={classes!("mockup-code", class)}>{children}</div>
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
    async fn test_code_mockup_basic() {
        let html = rsx! {
            <CodeMockup>
                <pre data-prefix="$"><code>"npm i daisyui"</code></pre>
            </CodeMockup>
        };
        assert!(html.contains(r#"class="mockup-code""#));
        assert!(html.contains(r#"data-prefix="$""#));
        assert!(html.contains("<code>"));
    }

    #[tokio::test]
    async fn test_code_mockup_custom_class() {
        let html = rsx! {
            <CodeMockup class="bg-primary">"X"</CodeMockup>
        };
        assert!(html.contains("bg-primary"));
    }
}
