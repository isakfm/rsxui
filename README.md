# RsxUI

**JSX-like declarative UI for Rust** — Build beautiful web interfaces with Rust's expressiveness.

RsxUI combines the familiar JSX syntax with Rust's type safety. Write HTML-like markup directly in Rust, compile to HTML strings, and use with any web framework (Axum, Actix-Web, etc.). No JavaScript runtime required.

[![MIT License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.94-orange.svg)](https://www.rust-lang.org)

---

## Features

- **JSX-like Syntax** — Familiar HTML-in-Rust syntax with dynamic interpolation
- **Type-Safe Components** — Props structs with compile-time checking via `bon`
- **Zero Runtime** — Pure compile-time macro expansion, outputs HTML strings
- **DaisyUI Components** — Pre-built, beautiful DaisyUI based components
- **Framework Agnostic** — Works with any Rust web framework
- **HTMX Ready** — Built-in HTMX & AlpineJS attribute support for server-side interactivity
- **ARIA/Accessibility** — Automatic accessibility attribute handling

---

## Crate Architecture

```
rsxui/
├── rsx/           # Core rendering engine
├── rsx-macros/    # Procedural macros (rsx!, classes!, #[ui], #[component])
└── rsxui/         # DaisyUI component library
```

| Crate | Description |
|-------|-------------|
| `rsx` | Core HTML rendering, escaping, and buffer management |
| `rsx-macros` | `rsx!`, `classes!`, `#[ui]`, `#[component]` macros |
| `rsxui` | Pre-built DaisyUI components (Button, Card, Input, etc.) |

---

## Quick Start

### Installation

```toml
[dependencies]
rsx = { path = "rsx" }
rsx-macros = { path = "rsx-macros" }
rsxui = { path = "rsxui" }
```

### Basic Usage

```rust
use rsx_macros::rsx;
use rsxui::components::{Button, Color, Size};

// Simple HTML
let html = rsx! {
    <div class="container">
        <h1>"Hello, World!"</h1>
    </div>
};

// Dynamic content
let user = "Alice";
let html = rsx! {
    <div>
        <p>"Welcome, {user}!"</p>
        <Button label="Get Started" color=Color::Primary />
    </div>
};
```

---

## Core Macros

### `rsx!` — HTML Rendering

The main macro for rendering HTML with Rust expressions:

```rust
use rsx_macros::rsx;

let items = vec!["Apples", "Bananas", "Oranges"];

let html = rsx! {
    <div class="card">
        <h2>"Shopping List"</h2>
        <ul>
            {items.iter().map(|item| rsx! {
                <li>{item}</li>
            }))}
        </ul>
    </div>
};
```

### `classes!` — Class Composition

Clean CSS class composition with conditional logic:

```rust
use rsx_macros::classes;

let is_active = true;
let size = "lg";

let class = classes!(
    "btn",
    is_active.then_some("btn-active"),
    size,
);

// Result: "btn btn-active lg"
```

### `#[ui]` — Component with All Attributes

The `#[ui]` attribute generates a component with automatic support for HTML, HTMX, Alpine.js, and ARIA attributes that is accessible via props:

```rust
use rsx_macros::ui;

#[ui]
fn Greeting(message: String) -> String {
    rsx! {
        <div class="greeting" {props.render_attrs()}>{props.message}</div>
    }
}

// Usage with automatic attributes
let html = rsx! {
    <Greeting
        message="Hello!"
        id="main-greeting"
        hx_get="/api/greet"
        onclick="handleClick()"
    />
};
```

### `#[component]` — Simple Component

For simpler components without auto-generated attributes:

```rust
use rsx_macros::{component, rsx};

#[component]
pub fn Card(title: String, content: String) -> String {
    rsx! {
        <div class="card">
            <h3>{title}</h3>
            <p>{content}</p>
        </div>
    }
}
```

---

## Utility Functions

### `class_if(condition, class)`

Returns the class string when condition is true:

```rust
use rsxui::components::class_if;

let html = rsx! {
    <button class={class_if(is_disabled, "btn-disabled")}>
        "Submit"
    </button>
};
```

### `attr_if(name, value)`

Returns `name="value"` when value is `Some`:

```rust
use rsxui::components::attr_if;

let placeholder: Option<String> = Some("Enter name".to_string());
let html = rsx! {
    <input
        type="text"
        {attr_if("placeholder", &placeholder)}
    />
};
```

### `show_if(condition, attr)`

Returns a boolean attribute (e.g. `disabled`, `checked`) when condition is true:

```rust
use rsxui::components::show_if;

let html = rsx! {
    <button {show_if(is_disabled, "disabled")}>"Submit"</button>
    <input type="checkbox" {show_if(is_checked, "checked")} />
};
```

---

## DaisyUI Components

### Implemented Components

| Component | Description |
|-----------|-------------|
| `Accordion` | Group of collapsible items with radio inputs or details |
| `Alert` | Message alerts with color and soft variants |
| `Avatar` | User avatars with status indicators and groups |
| `Badge` | Label/badge with color and size variants |
| `Button` | DaisyUI button with variants, sizes, and loading state |
| `Card` | Container with figure, body, title, and actions |
| `Chat` | Chat bubbles with placement, colors, header/footer |
| `Checkbox` | Checkbox input with color and size variants |
| `Collapse` | Collapsible content with arrow/plus modifiers |
| `Countdown` | Animated number countdown with CSS transitions |
| `Diff` | Side-by-side image/text comparison with slider |
| `Divider` | Content divider with text, colors, and orientation |
| `Drawer` | Responsive side navigation panel |
| `Fieldset` | Container for grouping form elements with title |
| `FileInput` | File upload input with color and size variants |
| `Filter` | Tab-style filter buttons |
| `Input` | Text input with color and size variants |
| `Label` | Form label and floating label |
| `Menu` | Navigation menu with active/focus/disabled states |
| `Radio` | Radio button with color and size variants |
| `Range` | Range slider with color and size variants |
| `Rating` | Star rating with half stars, custom masks, and read-only mode |
| `Select` | Dropdown select with color and size variants |
| `Textarea` | Multi-line text input with color and size variants |
| `Toggle` | Toggle switch with color and size variants |
| `Validator` | Input validation wrapper with hint messages |


### Component Examples

#### Button

```rust
use rsxui::components::{Button, Color, Size, ButtonStyle};

// Primary button
rsx! { <Button label="Click me" color=Color::Primary /> }

// Sizes
rsx! { <Button label="Small" size=Size::Sm /> }
rsx! { <Button label="Large" size=Size::Lg /> }

// Styles
rsx! { <Button label="Outline" style=ButtonStyle::Outline /> }
rsx! { <Button label="Soft" style=ButtonStyle::Soft /> }

// With icons and loading
rsx! { <Button label="Loading..." loading=true /> }
```

#### Card

```rust
use rsxui::components::{Card, CardBody, CardTitle, Color};

// Basic card
rsx! {
    <Card>
        <CardBody>
            <CardTitle>"Welcome"</CardTitle>
            <p>"This is a card component."</p>
        </CardBody>
    </Card>
};
```

#### Input

```rust
use rsxui::components::{Input, Color};

// Text input
rsx! {
    <Input
        placeholder="Enter your email"
        color=Color::Primary
    />
};
```

---

## Roadmap

### Implemented Components

- [x] **Accordion** — Group of collapsible items (radio or details)
- [x] **Alert** — Message alerts with color and soft variants
- [x] **Avatar** — User avatars with status and groups
- [x] **Badge** — Label/badge with color and size variants
- [x] **Button** — DaisyUI button with variants, sizes, loading state
- [x] **Card** — Container with figure, body, title, and actions
- [x] **Chat** — Chat bubbles with placement, colors, header/footer
- [x] **Checkbox** — Checkbox input with color and size variants
- [x] **Collapse** — Collapsible content with arrow/plus modifiers
- [x] **Countdown** — Animated number countdown with CSS transitions
- [x] **Diff** — Side-by-side image/text comparison with slider
- [x] **Divider** — Content divider with text, colors, and orientation
- [x] **Drawer** — Responsive side navigation panel
- [x] **Fieldset** — Container for grouping form elements with title
- [x] **File Input** — File upload input with color and size variants
- [x] **Filter** — Tab-style filter buttons
- [x] **Input** — Text input with color and size variants
- [x] **Label** — Form label and floating label
- [x] **Menu** — Navigation menu with active/focus/disabled states
- [x] **Radio** — Radio button with color and size variants
- [x] **Range** — Range slider with color and size variants
- [x] **Rating** — Star rating with half stars, custom masks, and read-only mode
- [x] **Select** — Dropdown select with color and size variants
- [x] **Textarea** — Multi-line text input with color and size variants
- [x] **Toggle** — Toggle switch with color and size variants
- [x] **Validator** — Input validation wrapper with hint messages

### Planned Components

- [ ] **Breadcrumb** — Navigation breadcrumb
- [ ] **Carousel** — Image/content carousel
- [ ] **Container** — Responsive container
- [ ] **Dock** — Bottom navigation bar
- [ ] **Dropdown** — Dropdown menu
- [ ] **Fab** — Floating Action Button
- [ ] **Footer** — Page footer
- [ ] **Hero** — Hero section
- [ ] **Indicator** — Status indicator
- [ ] **Join** — Grouped elements
- [ ] **Kbd** — Keyboard key display
- [ ] **Loading** — Loading spinner/animation
- [ ] **Mask** — Image mask/shape
- [ ] **Modal** — Dialog modal
- [ ] **Mockup** — Device mockups (phone, window, etc.)
- [ ] **Navbar** — Navigation bar
- [ ] **Pagination** — Page navigation
- [ ] **Progress** — Progress bar
- [ ] **Skeleton** — Loading state placeholder
- [ ] **Stack** — Stacked elements
- [ ] **Stat** — Statistics display
- [ ] **Steps** — Step wizard
- [ ] **Swap** — Swap icon/content
- [ ] **Table** — Data table
- [ ] **Tabs** — Tab navigation
- [ ] **Timeline** — Timeline display
- [ ] **Toast** — Toast notifications
- [ ] **Tooltip** — Hover tooltip

### Infrastructure Goals

- [x] Documentation website — Live at http://localhost:3000
- [x] Component test coverage — Tests for all implemented components
- [ ] Comprehensive test coverage — Edge cases and integration tests
- [ ] IDE support — Rust Analyzer snippets
- [ ] Crates.io release — Publish to crates.io

---

## Web Framework Integration

### Axum Example

```rust
use axum::{Router, routing::get, response::Html};
use rsx_macros::rsx;

async fn home() -> Html<String> {
    let html = rsx! {
        <div>
            <h1>"Welcome to RsxUI"</h1>
            <p>"Server-rendered with Axum"</p>
        </div>
    };
    Html(html)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(home));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
```

---

## License

MIT License. See [LICENSE](LICENSE) for details.

---

<p align="center">
  Built with ❤️ for the Rust community
</p>
