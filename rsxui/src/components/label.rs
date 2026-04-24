// Label Component
// Based on DaisyUI Label: https://daisyui.com/components/label/

use rsx_macros::{classes, component, rsx};

// ============================================
// Label - Input label wrapper
// ============================================

#[component]
pub fn Label(text: String, #[builder(default)] class: String, children: String) -> String {
    rsx! {
        <label class={classes!("label", class)}>
            <span class="label-text">{text}</span>
            {children}
        </label>
    }
}

// ============================================
// FloatingLabel - Floating label for input
// ============================================

#[component]
pub fn FloatingLabel(text: String, #[builder(default)] class: String, children: String) -> String {
    rsx! {
        <label class={classes!("floating-label", class)}>
            {children}
            <span>{text}</span>
        </label>
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
    async fn test_label_basic() {
        let html = rsx! {
            <Label text="Email">"<input type=\"email\" />"</Label>
        };
        assert!(html.contains("class=\"label\""));
        assert!(html.contains("label-text"));
        assert!(html.contains("Email"));
        assert!(html.contains("input"));
    }

    #[tokio::test]
    async fn test_floating_label() {
        let html = rsx! {
            <FloatingLabel text="Username">"<input type=\"text\" class=\"input\" />"</FloatingLabel>
        };
        assert!(html.contains("class=\"floating-label\""));
        assert!(html.contains("Username"));
        assert!(html.contains("input"));
    }
}
