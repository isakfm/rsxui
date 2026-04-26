// Countdown Component
// Based on DaisyUI Countdown: https://daisyui.com/components/countdown/

use rsx::{classes, component, rsx};

// ============================================
// CountdownGroup - Wrapper for multiple countdown values
// ============================================

#[component]
pub fn CountdownGroup(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <span class={classes!("countdown", class)}>
            {children}
        </span>
    }
}

// ============================================
// CountdownValue - Single countdown digit (no wrapper)
// ============================================

#[component]
pub fn CountdownValue(
    value: u16,
    #[builder(default)] aria_label: String,
    #[builder(default)] digits: u8,
) -> String {
    let val_str = value.to_string();
    let aria = if aria_label.is_empty() {
        val_str.clone()
    } else {
        aria_label
    };

    let digits_style = if digits > 0 {
        format!(" --digits:{};", digits)
    } else {
        String::new()
    };

    rsx! {
        <span style={format!("--value:{};{}", value, digits_style)} aria-live="polite" aria-label={aria}>{val_str}</span>
    }
}

// ============================================
// Countdown - Standalone countdown with wrapper
// ============================================

#[component]
pub fn Countdown(
    value: u16,
    #[builder(default)] class: String,
    #[builder(default)] aria_label: String,
    #[builder(default)] digits: u8,
    #[builder(default)] dynamic: bool,
) -> String {
    let val_str = value.to_string();
    let aria = if aria_label.is_empty() {
        val_str.clone()
    } else {
        aria_label
    };

    let digits_style = if digits > 0 {
        format!(" --digits:{};", digits)
    } else {
        String::new()
    };

    let id = format!("countdown-{}-{}", value, rand::random::<u32>());

    if dynamic {
        let script = format!(
            r#"<script>
(function() {{
  var el = document.getElementById('{}');
  var value = {};
  function update() {{
    el.style.setProperty('--value', value);
    el.textContent = value;
    el.setAttribute('aria-label', value);
    if (value > 0) {{
      value--;
      setTimeout(update, 1000);
    }}
  }}
  update();
}})();
</script>"#,
            id, value
        );

        rsx! {
            <span class={classes!("countdown", class)}>
                <span id={id} style={format!("--value:{};{}", value, digits_style)} aria-live="polite" aria-label={aria}>{val_str}</span>
            </span>
            {script}
        }
    } else {
        rsx! {
            <span class={classes!("countdown", class)}>
                <span style={format!("--value:{};{}", value, digits_style)} aria-live="polite" aria-label={aria}>{val_str}</span>
            </span>
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
    async fn test_countdown_basic() {
        let html = rsx! {
            <Countdown value=42 />
        };
        assert!(html.contains("class=\"countdown\""));
        assert!(html.contains("--value:42"));
        assert!(html.contains("42"));
        assert!(html.contains("aria-live=\"polite\""));
    }

    #[tokio::test]
    async fn test_countdown_with_aria_label() {
        let html = rsx! {
            <Countdown value=60 aria_label="60 seconds remaining" />
        };
        assert!(html.contains("aria-label=\"60 seconds remaining\""));
    }

    #[tokio::test]
    async fn test_countdown_custom_class() {
        let html = rsx! {
            <Countdown value=10 class="text-5xl font-mono" />
        };
        assert!(html.contains("text-5xl"));
        assert!(html.contains("font-mono"));
    }

    #[tokio::test]
    async fn test_countdown_with_digits() {
        let html = rsx! {
            <Countdown value=5 digits=2 />
        };
        assert!(html.contains("--digits:2"));
    }

    #[tokio::test]
    async fn test_countdown_dynamic() {
        let html = rsx! {
            <Countdown value=10 dynamic=true />
        };
        assert!(html.contains("class=\"countdown\""));
        assert!(html.contains("--value:10"));
        assert!(html.contains("setTimeout"));
        assert!(html.contains("value--"));
    }

    #[tokio::test]
    async fn test_countdown_group() {
        let html = rsx! {
            <CountdownGroup class="font-mono text-2xl">
                <CountdownValue value=10 />
                "h "
                <CountdownValue value=24 />
                "m "
                <CountdownValue value=59 />
                "s"
            </CountdownGroup>
        };
        // Should have only ONE outer countdown wrapper
        let wrapper_count = html.matches("class=\"countdown").count();
        assert_eq!(wrapper_count, 1);
        // Should have 3 inner value spans
        let value_count = html.matches("--value:").count();
        assert_eq!(value_count, 3);
        assert!(html.contains("--value:10"));
        assert!(html.contains("--value:24"));
        assert!(html.contains("--value:59"));
        assert!(html.contains("h "));
        assert!(html.contains("m "));
        assert!(html.contains("s"));
    }

    #[tokio::test]
    async fn test_countdown_value_standalone() {
        let html = rsx! {
            <CountdownValue value=42 />
        };
        // No outer countdown wrapper
        assert!(!html.contains("class=\"countdown\""));
        assert!(html.contains("--value:42"));
        assert!(html.contains("42"));
    }
}
