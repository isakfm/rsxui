// Fieldset Component
// Based on DaisyUI Fieldset: https://daisyui.com/components/fieldset/

use rsx::{classes, component, rsx};

// ============================================
// Fieldset - Form field grouping
// ============================================

#[component]
pub fn Fieldset(
    title: String,
    #[builder(default)] description: String,
    #[builder(default)] class: String,
    children: String,
) -> String {
    if description.is_empty() {
        rsx! {
            <fieldset class={classes!("fieldset", class)}>
                <legend class="fieldset-legend">{title}</legend>
                {children}
            </fieldset>
        }
    } else {
        rsx! {
            <fieldset class={classes!("fieldset", class)}>
                <legend class="fieldset-legend">{title}</legend>
                {children}
                <p class="label">{description}</p>
            </fieldset>
        }
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
    async fn test_fieldset_basic() {
        let html = rsx! {
            <Fieldset title="Personal Info">"<input type=\"text\" />"</Fieldset>
        };
        assert!(html.contains("class=\"fieldset\""));
        assert!(html.contains("fieldset-legend"));
        assert!(html.contains("Personal Info"));
        assert!(html.contains("input"));
    }

    #[tokio::test]
    async fn test_fieldset_with_description() {
        let html = rsx! {
            <Fieldset title="Email" description="We will never share your email.">"<input type=\"email\" />"</Fieldset>
        };
        assert!(html.contains("fieldset-legend"));
        assert!(html.contains("Email"));
        assert!(html.contains("class=\"label\""));
        assert!(html.contains("We will never share your email."));
    }
}
