# RsxUI — AI Agent Context

> **Purpose**: This document provides the architectural context, coding patterns, and workflow conventions needed to work effectively on the RsxUI project.

---

## 1. Project Overview

RsxUI is a **JSX-like declarative UI library for Rust** that compiles to HTML strings at build time (zero JavaScript runtime). It provides:

- A `rsx!` macro for writing HTML-like markup inside Rust
- Pre-built DaisyUI 5 component wrappers (`rsxui` crate)
- Full support for HTML, HTMX, Alpine.js, ARIA, and event attributes
- Framework-agnostic output (works with Axum, Actix-Web, etc.)

---

## 2. Workspace Architecture

```
rsxui/
├── Cargo.toml          # Workspace manifest
├── rsx/                # Core rendering engine
├── rsx-macros/         # Procedural macros (rsx!, classes!, #[ui], #[component])
├── rsxui/              # DaisyUI component library
└── website/            # Documentation website (Axum + RsxUI)
```

| Crate | Role | Key Files |
|-------|------|-----------|
| `rsx` | Core HTML rendering, escaping, buffer management, traits | `src/lib.rs`, `src/render.rs`, `src/escape.rs`, `src/buffer.rs`, `src/attrs.rs`, `src/props.rs` |
| `rsx-macros` | Proc-macro crate (re-exported by `rsx`) | `src/lib.rs` |
| `rsxui` | DaisyUI component implementations | `src/components/*.rs` |
| `website` | Axum server serving component documentation | `src/main.rs`, `src/layout.rs`, `src/pages/*.rs` |

### Dependency Flow
```
rsxui  →  rsx  →  rsx-macros
website →  rsxui, rsx
```

`rsx-macros` is a `proc-macro = true` crate. All macros are re-exported through `rsx` so downstream crates only depend on `rsx`.

---

## 3. The `rsx!` Macro

### What it does
`rsx!` parses JSX-like syntax at compile time using the `rstml` crate and expands it into a `format!()` call that produces a `String`.

### Tag Name Rules
- **Lowercase tags** → HTML elements (`div`, `span`, `input`, etc.)
- **Uppercase tags** → Components (`Button`, `Card`, etc.)
- **Void elements** self-close automatically: `area`, `base`, `br`, `col`, `embed`, `hr`, `img`, `input`, `link`, `meta`, `param`, `source`, `track`, `wbr`

### Syntax Patterns

```rust
use rsx::rsx;

// Static HTML
let html = rsx! { <div class="container">"Hello"</div> };

// Dynamic interpolation (auto-escaped)
let name = "<script>";
let html = rsx! { <p>{name}</p> };  // → &lt;script&gt;

// Rust expressions / blocks
let items = vec!["a", "b"];
let html = rsx! {
    <ul>
        {items.iter().map(|i| rsx! { <li>{i}</li> }).collect::<String>()}
    </ul>
};

// if/else as last expression (auto-wrapped to return String)
let html = rsx! {
    <div>
        {if show { "Visible" } else { "" }}
    </div>
};

// for loops as last expression (auto-wrapped)
let html = rsx! {
    <ul>
        {for item in items { rsx! { <li>{item}</li> } }}
    </ul>
};

// JSX-style @for (preferred — no nested rsx! needed)
let html = rsx! {
    <ul>
        @for item in &items {
            <li>{item}</li>
        }
    </ul>
};

// JSX-style @if with else
let html = rsx! {
    <div>
        @if show {
            <p>"Visible"</p>
        } else {
            <p>"Hidden"</p>
        }
    </div>
};

// JSX-style @if with else if chain
let html = rsx! {
    <div>
        @if status == 200 {
            <p>"OK"</p>
        } else if status == 404 {
            <p>"Not Found"</p>
        } else {
            <p>"Error"</p>
        }
    </div>
};
```

### Attribute Conventions

| Source | Target HTML | Notes |
|--------|-------------|-------|
| `class="foo"` | `class="foo"` | Static string |
| `hx_post="/api"` | `hx-post="/api"` | Underscore → dash for HTMX |
| `x_data="foo"` | `x-data="foo"` | Underscore → colon for Alpine.js |
| `_type="text"` | `type="text"` | Underscore prefix removes underscore |
| `as_="foo"` | `as="foo"` | Trailing underscore removed |
| `[disabled] = {bool}` | ` disabled` (conditional) | Bracket syntax = boolean conditional |
| `disabled = {true}` | ` disabled` | Known boolean attrs rendered without value |
| `disabled` (no value) | ` disabled` | Boolean attribute shorthand |
| `{some_expr}` as attr | Evaluated at runtime | Any expression returning a type that implements the expected trait |

### Custom Components in `rsx!`

When an uppercase tag is encountered, the macro generates a builder call:

```rust
// <Button label="Click" />
// expands to:
Button(ButtonProps::builder().label("Click").build()).await
```

Components are **always invoked with `.await`** — even though most components are synchronous, the macro expects an async signature.

---

## 4. Component Macros

### `#[component]` — Simple Component

Generates a `#[bon::Builder]` props struct. No automatic HTML/HTMX/ARIA attributes.

```rust
use rsx::{component, rsx};

#[component]
pub fn Card(title: String, content: String) -> String {
    rsx! {
        <div class="card">
            <h3>{title}</h3>
            <p>{content}</p>
        </div>
    }
}

// Usage
let html = rsx! { <Card title="Hello" content="World" /> };
```

### `#[ui]` — Full-Attribute Component

Generates a props struct **plus** all HTML, ARIA, HTMX, Alpine.js, and Event attributes. Also implements `RenderAttrs`.

```rust
use rsx::{rsx, ui};

#[ui]
pub fn Button(
    label: String,
    color: Color,
    #[builder(default)] size: Size,
    disabled: bool,
) -> String {
    rsx! {
        <button class="btn" {props.render_attrs()}>{props.label}</button>
    }
}

// Usage — all these attributes exist automatically:
let html = rsx! {
    <Button
        label="Save"
        color=Color::Primary
        id="save-btn"
        hx_post="/api/save"
        onclick="alert('saved')"
        aria_label="Save button"
    />
};
```

Key rules for `#[ui]`:
- Access props via the `props` parameter (e.g., `props.label`, `props.color`)
- Call `props.render_attrs()` to emit all auto-generated attributes as a string
- User-defined `bool` fields that match known HTML boolean attrs (e.g., `disabled`, `checked`) are rendered as boolean attributes automatically
- The struct derives `bon::Builder` and `Default`
- A `children: Option<String>` field is auto-added if not already present

> **All RsxUI DaisyUI components use `#[ui]`** for full HTML/HTMX/ARIA support. Use `#[component]` only for custom components that don't need those attributes.

### `#[props]` — Standalone Props Struct

Use when you need a props struct without a component function:

```rust
use rsx::props;

#[props]
pub struct AlertProps {
    message: String,
    #[builder(default)] color: Color,
}
```

---

## 5. Utility Macros

### `classes!` — CSS Class Composition

```rust
use rsx::classes;

let class = classes!(
    "btn",
    is_active.then_some("btn-active"),
    size,  // any Display type
    some_option,  // Options are filtered (None = empty)
);
// Result: "btn btn-active lg" (empty strings filtered)
```

### `raw!` — Raw HTML Passthrough

Bypasses escaping for trusted HTML strings:

```rust
use rsx::raw;

let html = rsx! {
    <div>{raw!("<em>Trusted</em>")}</div>
};
```

---

## 6. Core Types & Traits

### `Render` (`rsx::render::Render`)

```rust
pub trait Render {
    fn render(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
```

Implemented for: primitives, `String`, `str`, `Option<T>`, `Box<T>`, `&T`, `Cow<str>`, `()`.

### `Buffer` (`rsx::Buffer`)

Efficient HTML string builder. Used internally; rarely needed in components.

### `EscapeAttribute` (`rsx::EscapeAttribute`)

Trait for HTML attribute escaping. Used by the `rsx!` macro for dynamic attributes.

### `RenderAttrs` (`rsx::attrs::RenderAttrs`)

```rust
pub trait RenderAttrs {
    fn render_attrs(&self) -> String;
}
```

Implemented automatically by `#[ui]` for the generated props struct. Also implemented for `HtmlAttrs`, `AriaAttrs`, `HtmxAttrs`, `AlpineAttrs`, `EventAttrs`, and `AllAttrs`.

### `Props` (`rsx::props::Props`)

```rust
pub trait Props {
    type Builder;
    fn builder() -> Self::Builder;
}
```

---

## 7. DaisyUI Component Patterns (`rsxui` crate)

### File Structure
Each component lives in `rsxui/src/components/<name>.rs` and is re-exported from `rsxui/src/components/mod.rs`.

### Standard Component Anatomy

```rust
// 1. Imports
use super::{Color, Size};  // shared enums
use rsx::{classes, rsx, ui};
use rsx::attrs::RenderAttrs;

// 2. Domain enums (if needed)
#[derive(Debug, Clone, PartialEq, Default, EnumStringify)]
#[enum_stringify(case = "kebab", prefix = "btn-")]
pub enum ButtonStyle {
    #[default]
    #[enum_stringify(rename = "")]
    Default,
    Outline,
    // ...
}

// 3. Main component with #[ui]
#[ui]
pub fn Button(
    label: String,
    color: Color,
    #[builder(default)] style: ButtonStyle,
    disabled: bool,
) -> String {
    rsx! {
        <button
            class={classes!("btn", props.color.prefix("btn"), props.style)}
            {props.render_attrs()}
        >
            {props.label}
        </button>
    }
}

// 4. Tests
#[cfg(test)]
mod tests {
    use super::*;
    use rsx::rsx;

    #[tokio::test]
    async fn test_button_basic() {
        let html = rsx! { <Button label="Click" /> };
        assert!(html.contains("<button"));
        assert!(html.contains(">Click</button>"));
    }
}
```

### Shared Types (`rsxui::components`)

| Type | Purpose | Methods |
|------|---------|---------|
| `Color` | DaisyUI semantic colors | `.as_class()`, `.prefix("btn")` → `"btn-primary"` |
| `Size` | Reusable sizes | `.as_class()`, `.prefix("btn")` → `"btn-lg"` |

### Helper Functions (`rsxui::components`)

```rust
pub fn class_if(condition: bool, class: &str) -> String;
pub fn attr_if(name: &str, value: &Option<String>) -> String;  // " name=\"value\""
pub fn show_if(condition: bool, attr: &str) -> String;         // " disabled"
```

### Enum Conventions

Components often define domain-specific enums using `enum_stringify`:
- Use `#[enum_stringify(case = "kebab", prefix = "xxx-")]` for class-generating enums
- Use `#[enum_stringify(case = "lower")]` for value-generating enums (e.g., `type="button"`)
- Always derive `Debug, Clone, PartialEq, Default`
- The `Default` variant should map to an empty string via `#[enum_stringify(rename = "")]`

---

## 8. Attribute System Detail

The `#[ui]` macro auto-generates **flat fields** on the props struct (not nested structs):

```rust
// Generated struct looks like:
pub struct ButtonProps {
    // user fields
    pub label: String,
    pub color: Color,
    pub style: ButtonStyle,
    pub disabled: bool,
    pub children: Option<String>,
    // auto-generated fields
    pub id: Option<String>,
    pub class: Option<String>,
    pub hx_get: Option<String>,
    pub hx_post: Option<String>,
    pub aria_label: Option<String>,
    pub onclick: Option<String>,
    // ... 80+ more fields
}
```

The `render_attrs()` method renders all set optional fields as HTML attributes.

### Attribute Name Mapping in `rsx!`

The macro (`walk_attribute` in `rsx-macros/src/lib.rs`) applies these transforms:

```
hx_post       → hx-post       (underscore to dash)
x_data        → x-data        (underscore to dash)
aria_label    → aria-label    (underscore to dash)
_type         → type          (leading underscore stripped)
as_           → as            (trailing underscore stripped)
[disabled]    → conditional boolean attribute
```

---

## 9. Testing Conventions

- All component tests are `async` because `rsx!` invokes components with `.await`
- Use `#[tokio::test]`
- Test structural containment with `assert!(html.contains(...))`
- Test absence with `assert!(!html.contains(...))`
- For enum variants, iterate and assert each one
- Print HTML on failure: `println!("{}", html);`

```rust
#[tokio::test]
async fn test_all_colors() {
    for color in [Color::Primary, Color::Secondary, /* ... */] {
        let html = rsx! { <Button label="T" color=color /> };
        assert!(html.contains(&color.prefix("btn")), "Missing {:?}", color);
    }
}
```

---

## 10. Website Architecture

The `website` crate is an Axum server that serves documentation pages.

- **Routes**: Each component has a route like `GET /components/button`
- **Layout**: `layout.rs` provides a `drawer()` function that wraps content in a DaisyUI drawer with sidebar navigation
- **Pages**: `src/pages/<component>.rs` exports an async `page() -> String` function
- **Static assets**: Served from `static/` via `tower_http::ServeDir`

### Adding a New Component Page

1. Create `website/src/pages/<name>.rs`
2. Export `pub async fn page() -> String`
3. Add route in `website/src/main.rs`
4. Add sidebar link in `website/src/layout.rs`

---

## 11. Build & Run

```bash
# Run all tests
cargo test --workspace

# Run the documentation website
cargo run -p website
# → http://localhost:3000

# Build release
cargo build --release
```

### Rust Version
Minimum supported Rust version is **1.94** (due to workspace inheritance and edition 2024 features).

---

## 12. Common Tasks for AI Agents

### Adding a New DaisyUI Component

1. **Create file**: `rsxui/src/components/<name>.rs`
2. **Implement**: Use `#[ui]` so the component supports HTML/HTMX/ARIA attributes automatically
3. **Export**: Add `pub mod <name>;` and `pub use <name>::{...};` in `rsxui/src/components/mod.rs`
4. **Add tests**: Cover basic rendering, all enum variants, and attribute passthrough
5. **Add documentation page**: Create `website/src/pages/<name>.rs`, add route in `website/src/main.rs`, add sidebar link in `website/src/layout.rs`

### Modifying the `rsx!` Macro

- Edit `rsx-macros/src/lib.rs`
- Remember: `rsx-macros` is a proc-macro crate; it can only export proc macros
- Test changes with `cargo test -p rsx` and `cargo test -p rsxui`

### Adding New Auto-Generated Attributes to `#[ui]`

1. Add the field to `known_ui_attrs()` in `rsx-macros/src/lib.rs`
2. Add rendering logic in `generate_render_attrs_impl()` (if it's a bool with special handling)
3. Optionally add to `rsx::attrs::HtmlAttrs` / `AriaAttrs` / `HtmxAttrs` / `AlpineAttrs` / `EventAttrs` for manual struct use

### Adding a New Utility Macro

1. Implement in `rsx-macros/src/lib.rs`
2. Re-export in `rsx/src/lib.rs`
3. Add to `rsx/src/prelude.rs` if commonly used

---

## 13. Key Design Decisions

1. **Zero Runtime**: Everything compiles to `String` via `format!`. No virtual DOM, no runtime diffing.
2. **Async Components**: All components are async (`-> String`) even when synchronous, to allow future async data fetching inside components.
3. **Builder Pattern**: `bon` generates type-safe builders for all props structs.
4. **Flat Attributes**: `#[ui]` generates flat fields (not nested `props.html.id`) for simpler usage, with `render_attrs()` handling the grouping.
5. **HTML Escaping**: Dynamic content and attributes are automatically escaped. Use `raw!()` for trusted HTML.
6. **Framework Agnostic**: Output is just `String`. Integrate with Axum via `Html<String>`, Actix via `HttpResponse::Ok().body(html)`, etc.

---

## 14. File Index

| File | Purpose |
|------|---------|
| `rsx/src/lib.rs` | Re-exports macros and core types |
| `rsx/src/buffer.rs` | `Buffer` HTML string builder |
| `rsx/src/render.rs` | `Render` trait and impls |
| `rsx/src/escape.rs` | HTML escaping / `PreEscaped` |
| `rsx/src/attrs.rs` | Attribute structs and `RenderAttrs` trait |
| `rsx/src/props.rs` | `Props` and `Component` traits |
| `rsx/src/elements.rs` | Void element definitions |
| `rsx-macros/src/lib.rs` | All proc macros (`rsx!`, `classes!`, `#[ui]`, `#[component]`, etc.) |
| `rsxui/src/components/mod.rs` | Shared types (`Color`, `Size`) and component re-exports |
| `website/src/main.rs` | Axum router with all component pages |
| `website/src/layout.rs` | Drawer layout with sidebar navigation |

---

*Last updated: 2026-05-03*

---

## 15. Custom Node Architecture

The `@for` and `@if` syntax is implemented via rstml's `CustomNode` trait in `rsx-macros/src/lib.rs`.

### How it works

1. `RsxCustomNode` enum implements `rstml::node::CustomNode`
2. `RsxForExpr` and `RsxIfExpr` structs implement `ParseRecoverable` for parsing
3. `ParserConfig::custom_node::<RsxCustomNode>()` registers the custom node with rstml
4. `walk_nodes` handles `Node::Custom(RsxCustomNode::For(...))` and `Node::Custom(RsxCustomNode::If(...))` by generating the appropriate loop/conditional code

### Key structs

| Struct | Purpose |
|--------|---------|
| `RsxBlock` | Generic `{ body }` block containing parsed `Node<RsxCustomNode>` children |
| `RsxForExpr` | `@for pat in expr { block }` |
| `RsxIfExpr` | `@if condition { then } [else if ...]* [else { ... }]` |
| `RsxElseIf` | `else if condition { block }` |
| `RsxElse` | `else { block }` |
| `RsxCustomNode` | Enum wrapping `For` or `If` variants |
