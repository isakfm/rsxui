//! RsxUI Proc-Macros
//!
//! This crate provides the procedural macros for RsxUI:
//! - `rsx!` - JSX-like HTML rendering
//! - `classes!` - CSS class composition
//! - `raw!` - Raw HTML string passthrough
//! - `#[component]` - Component function attribute
//! - `#[props]` - Props struct attribute

use proc_macro::TokenStream;
use proc_macro2_diagnostics::Diagnostic;
use quote::{quote, ToTokens};
use rstml::node::{KeyedAttribute, Node, NodeAttribute, NodeElement, NodeName};
use rstml::{Parser, ParserConfig};
use std::collections::HashSet;
use syn::{FnArg, ItemFn, ItemStruct};

#[proc_macro]
pub fn rsx(tokens: TokenStream) -> TokenStream {
    rsx_inner(tokens, false).into()
}

#[proc_macro]
pub fn rsx_ide(tokens: TokenStream) -> TokenStream {
    rsx_inner(tokens, true).into()
}

fn empty_elements_set() -> HashSet<&'static str> {
    [
        "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param",
        "source", "track", "wbr",
    ]
    .into_iter()
    .collect()
}

fn rsx_inner(tokens: TokenStream, ide_helper: bool) -> proc_macro2::TokenStream {
    let config = ParserConfig::new()
        .recover_block(true)
        .element_close_use_default_wildcard_ident(true)
        .always_self_closed_elements(empty_elements_set());

    let parser = Parser::new(config);
    let (nodes, errors) = parser.parse_recoverable(tokens).split_vec();
    process_nodes(ide_helper, &nodes, errors)
}

fn process_nodes(
    ide_helper: bool,
    nodes: &[Node],
    errors: Vec<Diagnostic>,
) -> proc_macro2::TokenStream {
    let WalkNodesOutput {
        static_format,
        values,
        collected_elements: _elements,
        diagnostics,
    } = walk_nodes(nodes);

    let docs = if ide_helper { generate_docs() } else { vec![] };

    let errors = errors
        .into_iter()
        .map(|e| e.emit_as_expr_tokens())
        .chain(diagnostics);

    quote! {
        {
            #(#errors;)*
            #(#docs;)*
            format!(#static_format, #(#values),*)
        }
    }
}

fn generate_docs() -> Vec<proc_macro2::TokenStream> {
    vec![]
}

#[derive(Default)]
struct WalkNodesOutput<'a> {
    static_format: String,
    values: Vec<proc_macro2::TokenStream>,
    diagnostics: Vec<proc_macro2::TokenStream>,
    collected_elements: Vec<&'a NodeName>,
}

impl<'a> WalkNodesOutput<'a> {
    fn extend(&mut self, other: WalkNodesOutput<'a>) {
        self.static_format.push_str(&other.static_format);
        self.values.extend(other.values);
        self.diagnostics.extend(other.diagnostics);
        self.collected_elements.extend(other.collected_elements);
    }
}

/// If the last statement of a block is an `if` without `else`, wrap the
/// then-branch with `.to_string()` and add `else { String::new() }` so
/// both branches return `String` regardless of the original type.
fn wrap_block_if_needed(stmts: &[syn::Stmt]) -> proc_macro2::TokenStream {
    if let Some(last) = stmts.last() {
        if let syn::Stmt::Expr(expr, _) = last {
            if let syn::Expr::If(if_expr) = expr {
                if if_expr.else_branch.is_none() {
                    let cond = &if_expr.cond;
                    let then_branch = &if_expr.then_branch;
                    let all_but_last = &stmts[..stmts.len() - 1];
                    return quote! {
                        #(#all_but_last)*
                        if #cond {
                            (#then_branch).to_string()
                        } else {
                            String::new()
                        }
                    };
                }
            }
        }
    }
    quote!(#(#stmts)*)
}

fn walk_nodes<'a>(nodes: &'a [Node]) -> WalkNodesOutput<'a> {
    let mut out = WalkNodesOutput::default();

    for node in nodes {
        match node {
            Node::Doctype(doctype) => {
                let value = doctype.value.to_token_stream_string();
                out.static_format.push_str(&format!("<!DOCTYPE {}>", value));
            }
            Node::Element(element) => {
                let name = element.name().to_string();

                if !is_component_tag_name(&name) {
                    out.static_format.push_str(&format!("<{}", name));

                    for attribute in element.attributes() {
                        match attribute {
                            NodeAttribute::Block(block) => {
                                out.static_format.push_str("{}");
                                out.values.push(block.to_token_stream());
                            }
                            NodeAttribute::Attribute(attribute) => {
                                let (static_format, value) = walk_attribute(attribute);
                                out.static_format.push_str(&static_format);
                                if let Some(value) = value {
                                    out.values.push(value);
                                }
                            }
                        }
                    }

                    if is_void_element(element.open_tag.name.to_string().as_str()) {
                        out.static_format.push_str(" />");
                        continue;
                    }
                    out.static_format.push('>');

                    let children_output = walk_nodes(&element.children);
                    out.extend(children_output);

                    out.static_format.push_str(&format!("</{}>", name));
                } else {
                    out.static_format.push_str("{}");
                    out.values
                        .push(CustomElement::new(element).to_token_stream());
                }
            }
            Node::Text(text) => {
                out.static_format.push_str(&text.value_string());
            }
            Node::RawText(text) => {
                out.static_format.push_str(&text.to_string_best());
            }
            Node::Fragment(fragment) => {
                let other_output = walk_nodes(&fragment.children);
                out.extend(other_output);
            }
            Node::Comment(comment) => {
                out.static_format.push_str("<!-- {} -->");
                out.values.push(comment.value.to_token_stream());
            }
            Node::Block(block) => {
                let block = block.try_block().unwrap();
                let stmts = &block.stmts;
                let value = wrap_block_if_needed(stmts);
                out.static_format.push_str("{}");
                out.values.push(value);
            }
        }
    }

    out
}

fn is_void_element(name: &str) -> bool {
    matches!(
        name,
        "area"
            | "base"
            | "br"
            | "col"
            | "embed"
            | "hr"
            | "img"
            | "input"
            | "link"
            | "meta"
            | "param"
            | "source"
            | "track"
            | "wbr"
    )
}

fn is_component_tag_name(name: &str) -> bool {
    name.starts_with(|c: char| c.is_ascii_uppercase())
}

fn walk_attribute(attribute: &KeyedAttribute) -> (String, Option<proc_macro2::TokenStream>) {
    let mut static_format = String::new();
    let key = match attribute.key.to_string().as_str() {
        "as_" => "as".to_string(),
        "_type" => "type".to_string(),
        k if k.starts_with("on") => k.to_string(),
        k if k.starts_with("hx_") => k.replace('_', "-"),
        k if k.starts_with("x_") || k.starts_with("x-") => {
            if k.contains('_') {
                k.replace('_', ":")
            } else {
                k.to_string()
            }
        }
        _ => attribute.key.to_string(),
    };

    match attribute.value() {
        Some(syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Str(value),
            ..
        })) => {
            static_format.push(' ');
            static_format.push_str(&key);
            static_format.push_str("=\"");
            static_format.push_str(&html_escape::encode_double_quoted_attribute(&value.value()));
            static_format.push('"');
            (static_format, None)
        }
        Some(syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Bool(value),
            ..
        })) => {
            static_format.push(' ');
            static_format.push_str(&key);
            static_format.push_str(&format!("=\"{}\"", value.value()));
            (static_format, None)
        }
        Some(syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Int(value),
            ..
        })) => {
            static_format.push(' ');
            static_format.push_str(&key);
            static_format.push_str("=\"");
            static_format.push_str(&value.token().to_string());
            static_format.push('"');
            (static_format, None)
        }
        Some(syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Float(value),
            ..
        })) => {
            static_format.push(' ');
            static_format.push_str(&key);
            static_format.push_str("=\"");
            static_format.push_str(&value.token().to_string());
            static_format.push('"');
            (static_format, None)
        }
        Some(value) => {
            if key.starts_with('[') && key.ends_with(']') {
                let inner_key = &key[1..key.len() - 1];
                let inner_key_lit = syn::LitStr::new(inner_key, proc_macro2::Span::call_site());
                let value_quote = quote! {
                    if #value {
                        format!(" {}", #inner_key_lit)
                    } else {
                        String::new()
                    }
                };
                (String::new(), Some(value_quote))
            } else if matches!(
                key.as_str(),
                "disabled"
                    | "readonly"
                    | "checked"
                    | "selected"
                    | "multiple"
                    | "autofocus"
                    | "autoplay"
                    | "controls"
                    | "loop"
                    | "muted"
                    | "playsinline"
            ) {
                static_format.push_str("{}");
                let value_quote = quote! {
                    if #value {
                        format!(" {}", #key)
                    } else {
                        String::new()
                    }
                };
                (static_format, Some(value_quote))
            } else {
                static_format.push(' ');
                static_format.push_str(&key);
                static_format.push_str("=\"");
                static_format.push_str("{}");
                static_format.push('"');
                let value_quote = quote! {
                    ::rsx::EscapeAttribute::escape_attribute(&#value)
                };
                (static_format, Some(value_quote))
            }
        }
        None => {
            if key.starts_with('[') && key.ends_with(']') {
                let inner_key = &key[1..key.len() - 1];
                static_format = inner_key.to_string();
                (static_format, None)
            } else if matches!(
                key.as_str(),
                "disabled"
                    | "readonly"
                    | "checked"
                    | "selected"
                    | "multiple"
                    | "autofocus"
                    | "autoplay"
                    | "controls"
                    | "loop"
                    | "muted"
                    | "playsinline"
            ) {
                static_format = key.to_string();
                (static_format, None)
            } else {
                static_format.push(' ');
                static_format.push_str(&key);
                static_format.push_str("=\"");
                static_format.push_str("{}");
                static_format.push('"');
                (static_format, Some(quote!(#key)))
            }
        }
    }
}

struct CustomElement<'e> {
    e: &'e NodeElement,
}

impl<'e> CustomElement<'e> {
    fn new(e: &'e NodeElement) -> Self {
        CustomElement { e }
    }
}

impl ToTokens for CustomElement<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = self.e.name();

        let children = &self.e.children;
        let mut chain = vec![quote! {
            ::rsx::props::props_builder(&#name)
        }];

        if !children.is_empty() {
            let c = process_nodes(false, children, vec![]);
            chain.push(quote! { .children(#c) });
        }

        chain.push({
            self.e
                .attributes()
                .iter()
                .filter_map(|a| match a {
                    NodeAttribute::Block(_block) => None,
                    NodeAttribute::Attribute(attribute) => {
                        let key = &attribute.key;
                        let value = attribute.value();
                        match value {
                            Some(syn::Expr::Lit(syn::ExprLit {
                                lit: syn::Lit::Str(s),
                                ..
                            })) => {
                                let s = s.value();
                                Some(quote! { .#key(::std::string::String::from(#s)) })
                            }
                            Some(v) => Some(quote! { .#key(#v) }),
                            None => Some(quote! { .#key(true) }),
                        }
                    }
                })
                .collect::<proc_macro2::TokenStream>()
        });

        chain.push(quote! { .build() });

        tokens.extend(quote! {
            #name(#(#chain)*).await
        });
    }
}

#[proc_macro]
pub fn classes(tokens: TokenStream) -> TokenStream {
    use syn::punctuated::Punctuated;
    use syn::Expr;
    use syn::Token;

    let parsed: Punctuated<Expr, Token![,]> =
        syn::parse_macro_input!(tokens with Punctuated::parse_terminated);
    let elems: Vec<_> = parsed.into_iter().collect();

    quote! {
        {
            [#(#elems.to_string()),*].into_iter().filter(|s| !s.is_empty()).collect::<Vec<_>>().join(" ")
        }
    }
    .into()
}

#[proc_macro]
pub fn raw(tokens: TokenStream) -> TokenStream {
    let lit: syn::LitStr = syn::parse_macro_input!(tokens as syn::LitStr);
    let value = lit.value();
    quote! {
        ::rsx::render::FormatWrapper::new(std::borrow::Cow::Borrowed(#value))
    }
    .into()
}

#[proc_macro_attribute]
pub fn component(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let comp = syn::parse_macro_input!(input as ComponentFn);
    quote! { #comp }.to_token_stream().into()
}

struct ComponentFn {
    item: ItemFn,
}

impl syn::parse::Parse for ComponentFn {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let item = input.parse::<ItemFn>()?;
        Ok(ComponentFn { item })
    }
}

struct ComponentConfig {
    include_attrs: bool,
}

/// Known UI attributes for #[ui] macro.
/// Each tuple is (field_name, html_attribute_name, is_bool).
fn known_ui_attrs() -> Vec<(&'static str, &'static str, bool)> {
    let mut attrs = vec![];
    // HTML attrs
    for (field, html, is_bool) in [
        ("id", "id", false),
        ("name", "name", false),
        ("class", "class", false),
        ("style", "style", false),
        ("title", "title", false),
        ("lang", "lang", false),
        ("dir", "dir", false),
        ("tabindex", "tabindex", false),
        ("accesskey", "accesskey", false),
        ("hidden", "hidden", true),
        ("draggable", "draggable", false),
        ("translate", "translate", false),
        ("contenteditable", "contenteditable", false),
        ("spellcheck", "spellcheck", false),
    ] {
        attrs.push((field, html, is_bool));
    }
    // HTMX attrs
    for field in [
        "hx_get", "hx_post", "hx_put", "hx_delete", "hx_patch",
        "hx_trigger", "hx_target", "hx_swap", "hx_swap_oob",
        "hx_indicator", "hx_headers", "hx_ext", "hx_boost",
        "hx_history", "hx_include", "hx_exclude", "hx_replace_url",
        "hx_preserve", "hx_prompt", "hx_confirm", "hx_on",
        "hx_params", "hx_validate",
    ] {
        let html = field.replace('_', "-");
        attrs.push((field, Box::leak(html.into_boxed_str()), false));
    }
    // ARIA attrs
    for field in [
        "aria_label", "aria_labelledby", "aria_describedby", "aria_hidden",
        "aria_disabled", "aria_expanded", "aria_selected", "aria_checked",
        "aria_invalid", "aria_required", "aria_owns", "aria_controls",
        "aria_current", "aria_details", "aria_errormessage", "aria_flowto",
        "aria_keyshortcuts", "aria_live", "aria_relevant", "aria_atomic",
        "aria_colcount", "aria_colindex", "aria_colspan", "aria_rowcount",
        "aria_rowindex", "aria_rowspan", "aria_sort", "aria_description",
        "aria_busy",
    ] {
        let html = field.replace('_', "-");
        attrs.push((field, Box::leak(html.into_boxed_str()), false));
    }
    // Alpine attrs
    for field in [
        "x_data", "x_init", "x_bind", "x_on", "x_model", "x_show",
        "x_for", "x_transition", "x_effect", "x_cloak", "x_ignore",
        "x_text", "x_html", "x_ref", "x_mask", "x_scope", "x_teleport",
    ] {
        let html = field.replace('_', "-");
        attrs.push((field, Box::leak(html.into_boxed_str()), false));
    }
    // Event attrs
    for field in [
        "onclick", "ondblclick", "oncontextmenu", "onmousedown", "onmouseup",
        "onmouseenter", "onmouseleave", "onmousemove", "onmouseover", "onmouseout",
        "onkeydown", "onkeyup", "onkeypress", "onfocus", "onblur",
        "onfocusin", "onfocusout", "onchange", "oninput", "onsubmit",
        "onreset", "oninvalid", "oncut", "oncopy", "onpaste",
        "onload", "onerror", "onscroll",
    ] {
        attrs.push((field, field, false));
    }
    attrs
}

/// Standard HTML boolean attributes that user-defined `bool` fields can map to.
fn known_html_bool_attrs() -> std::collections::HashSet<&'static str> {
    [
        "disabled", "readonly", "required", "checked", "selected", "multiple",
        "autofocus", "autoplay", "controls", "loop", "muted", "playsinline",
        "hidden", "open", "reversed", "async", "defer", "nomodule", "ismap",
        "allowfullscreen", "formnovalidate", "novalidate", "spellcheck",
        "contenteditable", "draggable", "translate",
    ]
    .into_iter()
    .collect()
}

/// Generate `impl RenderAttrs` for a props struct with flat attribute fields.
fn generate_render_attrs_impl(
    props_name: &syn::Ident,
    user_field_names: &std::collections::HashSet<String>,
    user_bool_fields: &[(syn::Ident, String)],
) -> proc_macro2::TokenStream {
    let mut render_tokens = vec![];
    for (field, html, is_bool) in known_ui_attrs() {
        if user_field_names.contains(field) {
            continue;
        }
        let field_ident = syn::Ident::new(field, proc_macro2::Span::call_site());
        let html_lit = syn::LitStr::new(html, proc_macro2::Span::call_site());
        if field == "hidden" && is_bool {
            render_tokens.push(quote! {
                if self.#field_ident == Some(true) {
                    parts.push(format!(" {}", #html_lit));
                }
            });
        } else {
            render_tokens.push(quote! {
                if let Some(ref v) = self.#field_ident {
                    parts.push(format!(r#" {}="{}""#, #html_lit, v));
                }
            });
        }
    }
    // Auto-render user-defined bool fields that match known HTML boolean attrs
    for (field_ident, html_name) in user_bool_fields {
        let html_lit = syn::LitStr::new(html_name, proc_macro2::Span::call_site());
        render_tokens.push(quote! {
            if self.#field_ident {
                parts.push(format!(" {}", #html_lit));
            }
        });
    }
    quote! {
        impl ::rsx::attrs::RenderAttrs for #props_name {
            fn render_attrs(&self) -> String {
                let mut parts = Vec::new();
                #(#render_tokens)*
                parts.join("")
            }
        }
    }
}

fn expand_component_internal(
    comp: &ComponentFn,
    config: ComponentConfig,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let item = &comp.item;
    let name = &item.sig.ident;

    match item.sig.inputs.len() {
        0 => {
            let props_name =
                syn::Ident::new(&format!("{}Props", name), proc_macro2::Span::call_site());
            let builder_name = syn::Ident::new(
                &format!("{}Builder", props_name),
                proc_macro2::Span::call_site(),
            );

            let defs = if config.include_attrs {
                let mut extra_fields = vec![];
                for (field, _html, is_bool) in known_ui_attrs() {
                    let ident = syn::Ident::new(field, proc_macro2::Span::call_site());
                    let ty = if is_bool || field == "hidden" {
                        quote! { pub #ident: Option<bool> }
                    } else if field == "tabindex" || field.starts_with("aria_col") {
                        quote! { pub #ident: Option<i32> }
                    } else {
                        quote! { pub #ident: Option<String> }
                    };
                    extra_fields.push(ty);
                }
                let render_impl = generate_render_attrs_impl(&props_name, &std::collections::HashSet::new(), &[]);
                quote! {
                    #[derive(::rsx::bon::Builder, Default)]
                    pub struct #props_name {
                        #(#extra_fields,)*
                        pub children: Option<String>,
                    }

                    impl ::rsx::props::Props for #props_name {
                        type Builder = #builder_name;
                        fn builder() -> Self::Builder {
                            #props_name::builder()
                        }
                    }

                    #render_impl
                }
            } else {
                quote! {
                    #[derive(::rsx::bon::Builder, Default)]
                    pub struct #props_name {}

                    impl ::rsx::props::Props for #props_name {
                        type Builder = #builder_name;
                        fn builder() -> Self::Builder {
                            #props_name::builder()
                        }
                    }
                }
            };
            (defs, quote! { _props: #props_name })
        }
        1 if matches!(
            item.sig.inputs.first().unwrap(),
            syn::FnArg::Typed(arg) if {
                let ty = &arg.ty;
                if let syn::Type::Path(p) = ty.as_ref() {
                    p.path.segments.last().map(|s| s.ident.to_string()) == Some(format!("{}Props", name))
                } else {
                    false
                }
            }
        ) =>
        {
            let props = item.sig.inputs.first().unwrap();
            (quote! {}, props.to_token_stream())
        }
        _ => {
            let field_defs: Vec<_> = item
                .sig
                .inputs
                .iter()
                .filter_map(|i| match i {
                    FnArg::Receiver(_) => None,
                    FnArg::Typed(t) => Some(t.clone()),
                })
                .collect();

            let user_field_names: std::collections::HashSet<String> = field_defs
                .iter()
                .filter_map(|t| {
                    if let syn::Pat::Ident(ident) = t.pat.as_ref() {
                        Some(ident.ident.to_string())
                    } else {
                        None
                    }
                })
                .collect();

            let has_children = user_field_names.contains("children");

            // Detect user-defined bool fields that match known HTML boolean attrs
            let known_bool_attrs = known_html_bool_attrs();
            let user_bool_fields: Vec<(syn::Ident, String)> = field_defs
                .iter()
                .filter_map(|t| {
                    if let syn::Pat::Ident(ident) = t.pat.as_ref() {
                        let name = ident.ident.to_string();
                        if known_bool_attrs.contains(name.as_str()) {
                            // Check if type is `bool`
                            if let syn::Type::Path(p) = t.ty.as_ref() {
                                if p.path.segments.last().map(|s| s.ident == "bool").unwrap_or(false) {
                                    let html_name = name.replace('_', "-");
                                    return Some((ident.ident.clone(), html_name));
                                }
                            }
                        }
                    }
                    None
                })
                .collect();

            let field_defs_tokens: Vec<_> =
                field_defs.iter().map(|t| {
                    let attrs = &t.attrs;
                    let pat = &t.pat;
                    let ty = &t.ty;
                    let has_builder_attr = attrs.iter().any(|attr| attr.path().is_ident("builder"));
                    let is_option = if let syn::Type::Path(p) = ty.as_ref() {
                        p.path.segments.last().map(|s| s.ident == "Option").unwrap_or(false)
                    } else {
                        false
                    };
                    if has_builder_attr {
                        quote! { #(#attrs)* pub #pat: #ty }
                    } else if is_option {
                        quote! { pub #pat: #ty }
                    } else {
                        quote! { #[builder(default)] pub #pat: #ty }
                    }
                }).collect();

            let field_names: Vec<_> = item
                .sig
                .inputs
                .iter()
                .filter_map(|i| match i {
                    FnArg::Receiver(_) => None,
                    FnArg::Typed(t) => Some(&t.pat),
                })
                .collect();

            let props_name =
                syn::Ident::new(&format!("{}Props", name), proc_macro2::Span::call_site());
            let builder_name = syn::Ident::new(
                &format!("{}Builder", props_name),
                proc_macro2::Span::call_site(),
            );

            let defs = if config.include_attrs {
                let children_field = if has_children {
                    quote! {}
                } else {
                    quote! { pub children: Option<String>, }
                };

                // Generate flat HTML/HTMX/ARIA/Alpine/Event fields, skipping conflicts
                let mut extra_fields = vec![];

                for (field, _html, is_bool) in known_ui_attrs() {
                    if !user_field_names.contains(field) {
                        let ident = syn::Ident::new(field, proc_macro2::Span::call_site());
                        let ty = if is_bool || field == "hidden" {
                            quote! { pub #ident: Option<bool> }
                        } else if field == "tabindex" || field.starts_with("aria_col") {
                            quote! { pub #ident: Option<i32> }
                        } else {
                            quote! { pub #ident: Option<String> }
                        };
                        extra_fields.push(ty);
                    }
                }

                let render_impl = generate_render_attrs_impl(&props_name, &user_field_names, &user_bool_fields);

                quote! {
                    #[derive(::rsx::bon::Builder, Default)]
                    pub struct #props_name {
                        #(#field_defs_tokens,)*
                        #children_field
                        #(#extra_fields,)*
                    }

                    impl ::rsx::props::Props for #props_name {
                        type Builder = #builder_name;
                        fn builder() -> Self::Builder {
                            #props_name::builder()
                        }
                    }

                    #render_impl
                }
            } else {
                quote! {
                    #[derive(::rsx::bon::Builder)]
                    pub struct #props_name {
                        #(#field_defs_tokens),*
                    }

                    impl ::rsx::props::Props for #props_name {
                        type Builder = #builder_name;
                        fn builder() -> Self::Builder {
                            #props_name::builder()
                        }
                    }
                }
            };

            let args = if config.include_attrs {
                quote! { #props_name { #(#field_names),* , .. }: #props_name }
            } else {
                quote! { #props_name { #(#field_names),* }: #props_name }
            };
            (defs, args)
        }
    }
}

impl ToTokens for ComponentFn {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let item = &self.item;
        let name = &item.sig.ident;

        let (defs, args) = expand_component_internal(
            self,
            ComponentConfig {
                include_attrs: false,
            },
        );

        let body = &item.block;
        let output = &item.sig.output;
        let vis = &item.vis;

        tokens.extend(quote! {
            #defs
            #[allow(non_snake_case)]
            #vis async fn #name(#args) #output {
                #body
            }
        });
    }
}

#[proc_macro_attribute]
pub fn props(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let props_struct = syn::parse_macro_input!(input as PropsStruct);
    quote! { #props_struct }.to_token_stream().into()
}

struct PropsStruct {
    name: syn::Ident,
    item: ItemStruct,
}

impl syn::parse::Parse for PropsStruct {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let item = input.parse::<ItemStruct>()?;
        let name = item.ident.clone();
        Ok(PropsStruct { name, item })
    }
}

impl ToTokens for PropsStruct {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let item = &self.item;

        let builder_name =
            syn::Ident::new(&format!("{}Builder", name), proc_macro2::Span::call_site());

        tokens.extend(quote! {
            #[derive(::rsx::bon::Builder, Default)]
            #item

            impl ::rsx::props::Props for #name {
                type Builder = #builder_name;
                fn builder() -> Self::Builder {
                    #name::builder()
                }
            }
        });

        let has_attributes = item.fields.iter().any(|field| {
            field.ident.as_ref().map(|i| i.to_string()) == Some("attributes".to_string())
        });

        if has_attributes {
            tokens.extend(quote! {
                impl #builder_name {
                    pub fn push_attr<A: std::fmt::Display>(mut self, attr: A) -> Self {
                        self.props.attributes.push_str(&format!("{} ", attr));
                        self
                    }
                }
            });
        }
    }
}

/// UI component macro for DaisyUI components.
///
/// This macro generates a Props struct with all attribute groups:
/// - HTML attributes (id, class, style, etc.)
/// - ARIA attributes (aria-label, aria-disabled, etc.)
/// - HTMX attributes (hx-get, hx-post, etc.)
/// - Alpine.js attributes (x-data, x-on, etc.)
/// - Event handlers (onclick, onchange, etc.)
///
/// # Example
///
/// ```ignore
/// #[ui]
/// fn Button(label: String, color: Color) -> String {
///     // props.html.id, props.htmx.hx_post, props.events.onclick, etc. available
///     format!(r#"<button{} class="btn btn-{}">{}</button>"#,
///         props.render_attrs(), color, props.label)
/// }
/// ```
#[proc_macro_attribute]
pub fn ui(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let comp = syn::parse_macro_input!(input as ComponentFn);
    expand_ui_macro(&comp)
}

fn expand_ui_macro(comp: &ComponentFn) -> TokenStream {
    let item = &comp.item;
    let name = &item.sig.ident;

    let (defs, _) = expand_component_internal(
        comp,
        ComponentConfig {
            include_attrs: true,
        },
    );

    let body = &item.block;
    let vis = &item.vis;

    let props_name = syn::Ident::new(&format!("{}Props", name), proc_macro2::Span::call_site());

    quote! {
        #defs
        #[allow(non_snake_case)]
        #vis async fn #name(props: #props_name) -> String {
            #body
        }
    }
    .into()
}
