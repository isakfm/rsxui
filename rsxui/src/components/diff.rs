// Diff Component
// Based on DaisyUI Diff: https://daisyui.com/components/diff/

use rsx_macros::{classes, component, rsx};

// ============================================
// DiffItem1 - First comparison item
// ============================================

#[component]
pub fn DiffItem1(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div class={classes!("diff-item-1", class)} role="img" tabindex="0">
            {children}
        </div>
    }
}

// ============================================
// DiffItem2 - Second comparison item
// ============================================

#[component]
pub fn DiffItem2(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div class={classes!("diff-item-2", class)} role="img">
            {children}
        </div>
    }
}

// ============================================
// DiffResizer - The resizer control
// ============================================

#[component]
pub fn DiffResizer() -> String {
    rsx! {
        <div class="diff-resizer" />
    }
}

// ============================================
// Diff - Side-by-side comparison container
// ============================================

#[component]
pub fn Diff(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <figure class={classes!("diff", class)} tabindex="0">
            {children}
        </figure>
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
    async fn test_diff_basic() {
        let html = rsx! {
            <Diff class="aspect-16/9">
                <DiffItem1>
                    <img src="a.jpg" alt="Before" />
                </DiffItem1>
                <DiffItem2>
                    <img src="b.jpg" alt="After" />
                </DiffItem2>
                <DiffResizer />
            </Diff>
        };
        assert!(html.contains(r#"<figure class="diff aspect-16/9" tabindex="0">"#));
        assert!(html.contains(r#"<div class="diff-item-1" role="img" tabindex="0">"#));
        assert!(html.contains(r#"<div class="diff-item-2" role="img">"#));
        assert!(html.contains(r#"<div class="diff-resizer">"#));
        assert!(html.contains("a.jpg"));
        assert!(html.contains("b.jpg"));
    }

    #[tokio::test]
    async fn test_diff_text_content() {
        let html = rsx! {
            <Diff>
                <DiffItem1>
                    <div class="bg-primary">"BEFORE"</div>
                </DiffItem1>
                <DiffItem2>
                    <div class="bg-base-200">"AFTER"</div>
                </DiffItem2>
                <DiffResizer />
            </Diff>
        };
        assert!(html.contains("BEFORE"));
        assert!(html.contains("AFTER"));
    }
}
