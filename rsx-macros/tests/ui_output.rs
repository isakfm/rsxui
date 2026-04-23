//! Integration tests for `#[ui]` macro with DaisyUI components

use rsx_macros::{rsx, ui};

#[ui]
fn DaisyCard(heading: String) -> String {
    rsx! {
        <div class="card">
            <h2>{props.heading}</h2>
            <div class="body">default</div>
        </div>
    }
}

#[tokio::test]
async fn test_daisy_card() {
    let html = rsx! {
        <DaisyCard heading="My Card" />
    };
    assert!(html.contains("<div class=\"card\">"));
    assert!(html.contains("<h2>My Card</h2>"));
}