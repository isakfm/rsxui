//! Integration tests for custom components using `#[component]` macro

use rsx_macros::{component, rsx};

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

#[component]
fn ProductHeader(title: String) -> String {
    rsx! {
        <div class="product-header">
            <h3>{title}</h3>
        </div>
    }
}

#[component]
fn Products(products: Vec<Product>) -> String {
    rsx! {
        <div class="products">
            <ProductHeader title="Product List" />
            {
                for p in &products {
                    rsx! { <ProductItem item=p.clone() /> }
                }
            }
        </div>
    }
}

#[tokio::test]
async fn test_product_struct() {
    let product = Product::new(1, "Widget", 19.99);
    assert_eq!(product.id, 1);
    assert_eq!(product.name, "Widget");
    assert_eq!(product.price, 19.99);
}

#[tokio::test]
async fn test_product_item_component() {
    let product = Product::new(42, "Gadget", 29.99);
    let html = rsx! {
        <ProductItem item=product />
    };
    assert!(html.contains("data-id=\"42\""));
    assert!(html.contains("Gadget</span>"));
    assert!(html.contains("$29.99"));
}

#[tokio::test]
async fn test_product_header_component() {
    let html = rsx! {
        <ProductHeader title="Featured Products" />
    };
    assert!(html.contains("<div class=\"product-header\">"));
    assert!(html.contains("<h3>Featured Products</h3>"));
}

#[tokio::test]
async fn test_nested_products_component() {
    let products = vec![
        Product::new(1, "Apple", 1.99),
        Product::new(2, "Banana", 0.99),
        Product::new(3, "Orange", 2.49),
    ];
    let html = rsx! {
        <Products products=products />
    };
    println!("{}", html);
    assert!(html.contains("<div class=\"products\">"));
    assert!(html.contains("<div class=\"product-header\">"));
    assert!(html.contains("Apple</span>"));
    assert!(html.contains("Banana</span>"));
    assert!(html.contains("Orange</span>"));
}

#[tokio::test]
async fn test_products_with_header() {
    let products = vec![Product::new(1, "Test Product", 10.00)];
    let html = rsx! {
        <div>
            <Products products=products />
        </div>
    };
    println!("{}", html);
    assert!(html.contains("<div>"));
    assert!(html.contains("<div class=\"products\">"));
    assert!(html.contains("Product List</h3>"));
    assert!(html.contains("</div>"));
}

#[tokio::test]
async fn test_empty_products() {
    let products: Vec<Product> = vec![];
    let html = rsx! {
        <Products products=products />
    };
    assert!(html.contains("<div class=\"products\">"));
    assert!(html.contains("<div class=\"product-header\">"));
    assert!(html.contains("Product List</h3>"));
}

#[tokio::test]
async fn test_product_item_with_special_characters() {
    let product = Product::new(1, "Rock & Roll", 15.00);
    let html = rsx! {
        <ProductItem item=product />
    };
    assert!(html.contains("Rock & Roll"));
    assert!(html.contains("data-id=\"1\""));
}

#[tokio::test]
async fn test_multiple_products_rendering() {
    let products = vec![
        Product::new(1, "Item A", 10.0),
        Product::new(2, "Item B", 20.0),
    ];
    let html = rsx! {
        <Products products=products />
    };
    assert!(html.contains("data-id=\"1\""));
    assert!(html.contains("data-id=\"2\""));
    assert!(html.contains("Item A</span>"));
    assert!(html.contains("Item B</span>"));
    assert!(html.contains("$10"));
    assert!(html.contains("$20"));
}

#[tokio::test]
async fn test_product_price_formatting() {
    let products = vec![
        Product::new(1, "Cheap", 0.01),
        Product::new(2, "Expensive", 9999.99),
    ];
    let html = rsx! {
        <Products products=products />
    };
    assert!(html.contains("$0.01"));
    assert!(html.contains("$9999.99"));
}

#[tokio::test]
async fn test_for_loop_with_filter() {
    let products = vec![
        Product::new(1, "Apple", 1.99),
        Product::new(2, "Banana", 0.99),
        Product::new(3, "Cherry", 2.99),
    ];
    let html = rsx! {
        <div>
            {
                for p in &products {
                    if p.price > 1.0 {
                        rsx! { <ProductItem item=p.clone() /> }
                    }
                }
            }
        </div>
    };
    assert!(html.contains("Apple"));
    assert!(html.contains("Cherry"));
    assert!(!html.contains("Banana"));
}

#[tokio::test]
async fn test_for_loop_with_index() {
    let items = vec![
        "First".to_string(),
        "Second".to_string(),
        "Third".to_string(),
    ];
    let mut html_output = String::new();
    let mut idx = 0;

    html_output.push_str("<div>");
    for item in &items {
        let item_html = rsx! { <span data-index={idx}>{item}</span> };
        html_output.push_str(&item_html);
        idx += 1;
    }
    html_output.push_str("</div>");

    assert!(html_output.contains("data-index=\"0\""));
    assert!(html_output.contains("data-index=\"1\""));
    assert!(html_output.contains("data-index=\"2\""));
    assert!(html_output.contains("First"));
    assert!(html_output.contains("Second"));
    assert!(html_output.contains("Third"));
}

#[tokio::test]
async fn test_for_loop_empty_collection() {
    let empty: Vec<Product> = vec![];
    let html = rsx! {
        <div>
            {
                for p in &empty {
                    rsx! { <ProductItem item=p.clone() /> }
                }
            }
        </div>
    };
    assert!(html.contains("<div>"));
    assert!(html.contains("</div>"));
    assert!(!html.contains("product-item"));
}
