// Label Component
// Based on DaisyUI Label: https://daisyui.com/components/label/

use rsx::attrs::RenderAttrs;
use rsx::{classes, rsx, ui};

// ============================================
// Label - Input label wrapper
// ============================================

#[ui]
pub fn Label(text: String, #[builder(default)] class: String, children: String) -> String {
    rsx! {
        <label class={classes!("label", props.class)} {props.render_attrs()}>
            <span class="label-text">{props.text}</span>
            {props.children}
        </label>
    }
}

// ============================================
// FloatingLabel - Floating label for input
// ============================================

#[ui]
pub fn FloatingLabel(text: String, #[builder(default)] class: String, children: String) -> String {
    rsx! {
        <label class={classes!("floating-label", props.class)} {props.render_attrs()}>
            {props.children}
            <span>{props.text}</span>
        </label>
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
