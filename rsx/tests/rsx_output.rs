//! Integration tests for rsx! macro output (via `rsx` re-exports)

use rsx::{raw, rsx};

#[test]
fn test_simple_element() {
    let html = rsx! {
        <div>"Hello"</div>
    };
    assert_eq!(html, "<div>Hello</div>");
}

#[test]
fn test_element_with_attribute() {
    let html = rsx! {
        <div class="container">"Content"</div>
    };
    assert_eq!(html, r#"<div class="container">Content</div>"#);
}

#[test]
fn test_nested_elements() {
    let html = rsx! {
        <div>
            <span>"Nested"</span>
        </div>
    };
    assert_eq!(html, "<div><span>Nested</span></div>");
}

#[test]
fn test_void_element_self_closing() {
    let html = rsx! { <br /> };
    assert_eq!(html, "<br />");
}

#[test]
fn test_img_element() {
    let html = rsx! { <img src="test.png" alt="test" /> };
    assert!(html.contains("<img"));
    assert!(html.contains("alt=\"test\""));
}

#[test]
fn test_boolean_attribute_syntax() {
    let html = rsx! {
        <button disabled="true">"Click"</button>
    };
    assert_eq!(html, r#"<button disabled="true">Click</button>"#);
}

#[test]
fn test_dynamic_content() {
    let name = "World";
    let html = rsx! {
        <p>"Hello, {name}!"</p>
    };
    assert_eq!(html, "<p>Hello, World!</p>");
}

#[test]
fn test_raw_macro() {
    let html = raw!("<script>alert('xss')</script>");
    let result = format!("{}", html);
    assert_eq!(result, "<script>alert('xss')</script>");
}

#[test]
fn test_element_with_id() {
    let html = rsx! {
        <div id="main">"Content"</div>
    };
    assert_eq!(html, r#"<div id="main">Content</div>"#);
}

#[test]
fn test_multiple_siblings() {
    let html = rsx! {
        <div>
            <p>"First"</p>
            <p>"Second"</p>
            <p>"Third"</p>
        </div>
    };
    assert_eq!(html, "<div><p>First</p><p>Second</p><p>Third</p></div>");
}

#[test]
fn test_self_closing_tags() {
    let html = rsx! {
        <div>
            <img src="a.png" />
            <br />
            <hr />
        </div>
    };
    assert!(html.contains("<img"));
    assert!(html.contains("<br />"));
    assert!(html.contains("<hr />"));
}

#[test]
fn test_escaping_content() {
    let html = rsx! {
        <div>"&lt;script&gt;alert('xss')&lt;/script&gt;"</div>
    };
    assert!(html.contains("&lt;script&gt;"));
}

#[test]
fn test_mixed_content() {
    let count = 5;
    let html = rsx! {
        <div class="container">
            <h1>"Count: {count}"</h1>
            <p>"Items:"</p>
            <ul>
                <li>"Item 1"</li>
                <li>"Item 2"</li>
            </ul>
        </div>
    };
    assert!(html.contains("<div"));
    assert!(html.contains("Count: 5"));
    assert!(html.contains("<ul>"));
    assert!(html.contains("<li>Item 1</li>"));
    assert!(html.contains("<li>Item 2</li>"));
    assert!(html.contains("</div>"));
}

#[test]
fn test_h1_to_h6_headings() {
    let html = rsx! { <h1>"Title"</h1> };
    assert_eq!(html, "<h1>Title</h1>");

    let html = rsx! { <h2>"Subtitle"</h2> };
    assert_eq!(html, "<h2>Subtitle</h2>");

    let html = rsx! { <h3>"Heading"</h3> };
    assert_eq!(html, "<h3>Heading</h3>");
}

#[test]
fn test_unordered_list() {
    let html = rsx! {
        <ul>
            <li>"One"</li>
            <li>"Two"</li>
        </ul>
    };
    assert_eq!(html, "<ul><li>One</li><li>Two</li></ul>");
}

#[test]
fn test_ordered_list() {
    let html = rsx! {
        <ol>
            <li>"First"</li>
            <li>"Second"</li>
        </ol>
    };
    assert_eq!(html, "<ol><li>First</li><li>Second</li></ol>");
}

#[test]
fn test_table_structure() {
    let html = rsx! {
        <table>
            <tr>
                <td>"Cell"</td>
            </tr>
        </table>
    };
    assert!(html.contains("<table>"));
    assert!(html.contains("<tr>"));
    assert!(html.contains("<td>Cell</td>"));
    assert!(html.contains("</table>"));
}

#[test]
fn test_select_dropdown() {
    let html = rsx! {
        <select name="choice">
            <option value="a">"Option A"</option>
            <option value="b">"Option B"</option>
        </select>
    };
    assert!(html.contains("<select"));
    assert!(html.contains("<option value=\"a\">Option A</option>"));
    assert!(html.contains("<option value=\"b\">Option B</option>"));
}

#[test]
fn test_span_element() {
    let html = rsx! {
        <span class="highlight">"Important"</span>
    };
    assert_eq!(html, r#"<span class="highlight">Important</span>"#);
}

