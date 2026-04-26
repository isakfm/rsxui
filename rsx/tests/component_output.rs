//! Integration tests for custom components using `#[component]` macro (via `rsx` re-exports)

use rsx::{component, rsx};

#[component]
fn SimpleCard(title: String) -> String {
    rsx! {
        <div class="card">
            <h2>
                {title}
            </h2>
        </div>
    }
}

#[component]
fn CardWithChildren(title: String, children: String) -> String {
    rsx! {
        <div class="card">
            <h2>{title}</h2>
            <div class="body">{children}</div>
        </div>
    }
}

#[component]
fn Badge(text: String, color: String) -> String {
    rsx! {
        <span class={format!("badge badge-{}", color)}>{text}</span>
    }
}

#[component]
fn IconBadge(icon: String, count: i32) -> String {
    rsx! {
        <span class="badge"><i class={icon}>{count}</i></span>
    }
}

#[component]
fn ButtonComponent(label: String, variant: String) -> String {
    let variant_class = if variant.is_empty() {
        "btn".to_string()
    } else {
        format!("btn btn-{}", variant)
    };
    rsx! {
        <button class={variant_class}>{label}</button>
    }
}

#[component]
fn LinkComponent(href: String, text: String) -> String {
    rsx! {
        <a href={href} class="link">{text}</a>
    }
}

#[tokio::test]
async fn test_simple_card_component() {
    let html = rsx! {
        <SimpleCard title="Hello" />
    };
    assert!(html.contains("<div class=\"card\">"));
    assert!(html.contains("<h2>Hello</h2>"));
    assert!(html.contains("</div>"));
}

#[tokio::test]
async fn test_card_with_children() {
    let html = rsx! {
        <CardWithChildren title="Title">
            <LinkComponent href="/abc" text="ABC" />
        </CardWithChildren>
    };
    assert!(html.contains("<div class=\"card\">"));
    assert!(html.contains("<h2>Title</h2>"));
    assert!(html.contains("<a href=\"/abc\""));
    assert!(html.contains("class=\"link\""));
}

#[tokio::test]
async fn test_badge_component() {
    let html = rsx! {
        <Badge text="New" color="primary" />
    };
    assert!(html.contains("<span class=\"badge badge-primary\">"));
    assert!(html.contains("New</span>"));
}

#[tokio::test]
async fn test_icon_badge_component() {
    let html = rsx! {
        <IconBadge icon="fa-star" count=5 />
    };
    assert!(html.contains("<span class=\"badge\">"));
    assert!(html.contains("<i class=\"fa-star\">"));
    assert!(html.contains("5</i></span>"));
}

#[tokio::test]
async fn test_button_component_primary() {
    let html = rsx! {
        <ButtonComponent label="Click Me" variant="primary" />
    };
    assert!(html.contains("<button class=\"btn btn-primary\">"));
    assert!(html.contains("Click Me</button>"));
}

#[tokio::test]
async fn test_button_component_no_variant() {
    let html = rsx! {
        <ButtonComponent label="Default" variant="" />
    };
    assert!(html.contains("<button class=\"btn\">"));
    assert!(html.contains("Default</button>"));
}

#[tokio::test]
async fn test_link_component() {
    let html = rsx! {
        <LinkComponent href="/about" text="About Us" />
    };
    assert!(html.contains("<a href=\"/about\" class=\"link\">"));
    assert!(html.contains("About Us</a>"));
}

#[tokio::test]
async fn test_multiple_components() {
    let html = rsx! {
        <div>
            <SimpleCard title="Card 1" />
            <SimpleCard title="Card 2" />
            <SimpleCard title="Card 3" />
        </div>
    };
    assert!(html.contains("<div>"));
    assert!(html.contains("Card 1</h2>"));
    assert!(html.contains("Card 2</h2>"));
    assert!(html.contains("Card 3</h2>"));
    assert!(html.contains("</div>"));
}

#[tokio::test]
async fn test_badge_variants() {
    for color in &[
        "primary",
        "secondary",
        "accent",
        "neutral",
        "info",
        "success",
        "warning",
        "error",
    ] {
        let html = rsx! {
            <Badge text="Test" color=color.to_string() />
        };
        assert!(
            html.contains(&format!("badge-{}", color)),
            "Badge should contain badge-{} but got: {}",
            color,
            html
        );
    }
}

#[tokio::test]
async fn test_button_variants() {
    for variant in &["", "primary", "secondary", "ghost", "link"] {
        let html = rsx! {
            <ButtonComponent label="Click" variant=variant.to_string() />
        };
        if variant.is_empty() {
            assert!(
                html.contains("class=\"btn\">"),
                "Button should have btn class: {}",
                html
            );
        } else {
            assert!(
                html.contains(&format!("btn-{}", variant)),
                "Button should contain btn-{} but got: {}",
                variant,
                html
            );
        }
    }
}

// ========================================
// Nested Components Tests
// ========================================

#[derive(Debug, Clone, Default)]
pub struct Product {
    id: i32,
    name: String,
    price: f64,
}

impl Product {
    fn new(id: i32, name: &str, price: f64) -> Self {
        Self {
            id,
            name: name.to_string(),
            price,
        }
    }
}

#[component]
fn ProductItem(item: Product) -> String {
    rsx! {
        <div class="product-item" data-id={item.id}>
            <span class="name">{item.name}</span>
            <span class="price">${item.price}</span>
        </div>
    }
}

