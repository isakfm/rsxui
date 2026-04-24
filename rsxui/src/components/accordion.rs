// Accordion Component
// Based on DaisyUI Accordion: https://daisyui.com/components/accordion/

use enum_stringify::EnumStringify;
use rsx_macros::{classes, component, rsx};

// ============================================
// AccordionModifier - Accordion modifier
// ============================================

#[derive(Debug, Clone, PartialEq, Default, EnumStringify)]
#[enum_stringify(case = "kebab", prefix = "collapse-")]
pub enum AccordionModifier {
    #[default]
    #[enum_stringify(rename = "")]
    Default,
    Arrow,
    Plus,
}

// ============================================
// AccordionItem - Single accordion item with radio input
// ============================================

#[component]
pub fn AccordionItem(
    name: String,
    title: String,
    #[builder(default)] checked: bool,
    #[builder(default)] modifier: AccordionModifier,
    #[builder(default)] class: String,
    children: String,
) -> String {
    rsx! {
        <div class={classes!("collapse", modifier, class)}>
            <input type="radio" name={name} checked=checked />
            <div class="collapse-title">{title}</div>
            <div class="collapse-content">{children}</div>
        </div>
    }
}

// ============================================
// AccordionDetails - Single accordion item using details element
// ============================================

#[component]
pub fn AccordionDetails(
    name: String,
    title: String,
    #[builder(default)] open: bool,
    #[builder(default)] modifier: AccordionModifier,
    #[builder(default)] class: String,
    children: String,
) -> String {
    rsx! {
        <details class={classes!("collapse", modifier, class)} name={name} open={open}>
            <summary class="collapse-title">{title}</summary>
            <div class="collapse-content">{children}</div>
        </details>
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
    async fn test_accordion_basic() {
        let html = rsx! {
            <AccordionItem name="my-accordion" title="Item 1" checked=true>"Content 1"</AccordionItem>
        };
        assert!(html.contains("class=\"collapse\""));
        assert!(html.contains("type=\"radio\""));
        assert!(html.contains("name=\"my-accordion\""));
        assert!(html.contains("checked"));
        assert!(html.contains("collapse-title"));
        assert!(html.contains("Item 1"));
        assert!(html.contains("collapse-content"));
        assert!(html.contains("Content 1"));
    }

    #[tokio::test]
    async fn test_accordion_arrow() {
        let html = rsx! {
            <AccordionItem name="accordion-2" title="Item 1" modifier=AccordionModifier::Arrow>"Content"</AccordionItem>
        };
        assert!(html.contains("collapse-arrow"));
    }

    #[tokio::test]
    async fn test_accordion_plus() {
        let html = rsx! {
            <AccordionItem name="accordion-3" title="Item 1" modifier=AccordionModifier::Plus>"Content"</AccordionItem>
        };
        assert!(html.contains("collapse-plus"));
    }

    #[tokio::test]
    async fn test_accordion_custom_class() {
        let html = rsx! {
            <AccordionItem name="accordion-4" title="Item 1" class="bg-base-100 border border-base-300">"Content"</AccordionItem>
        };
        assert!(html.contains("bg-base-100"));
        assert!(html.contains("border-base-300"));
    }

    #[tokio::test]
    async fn test_accordion_details() {
        let html = rsx! {
            <AccordionDetails name="det-1" title="Details Item" open=true>"Details content"</AccordionDetails>
        };
        assert!(html.contains("details"));
        assert!(html.contains("name=\"det-1\""));
        assert!(html.contains("open"));
        assert!(html.contains("summary"));
        assert!(html.contains("collapse-title"));
        assert!(html.contains("Details Item"));
        assert!(html.contains("Details content"));
    }

    #[tokio::test]
    async fn test_accordion_multiple_items() {
        let html = rsx! {
            <div class="join join-vertical">
                <AccordionItem name="multi" title="First" checked=true class="join-item border border-base-300">"First content"</AccordionItem>
                <AccordionItem name="multi" title="Second" class="join-item border border-base-300">"Second content"</AccordionItem>
                <AccordionItem name="multi" title="Third" class="join-item border border-base-300">"Third content"</AccordionItem>
            </div>
        };
        assert!(html.contains("join-item"));
        assert!(html.contains("First"));
        assert!(html.contains("Second"));
        assert!(html.contains("Third"));
        // All items share the same radio name
        let count = html.matches("name=\"multi\"").count();
        assert_eq!(count, 3);
    }
}