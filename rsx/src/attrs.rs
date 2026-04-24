//! HTML, ARIA, HTMX, Alpine.js, and Event attributes for `#[ui]` macro.
//!
//! This module defines all attribute structures that can be auto-generated
//! into DaisyUI component props when using the `#[ui]` macro.

use std::collections::HashMap;

/// HTML5 Global Attributes
#[derive(Default, Debug, Clone)]
pub struct HtmlAttrs {
    pub id: Option<String>,
    pub name: Option<String>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub title: Option<String>,
    pub lang: Option<String>,
    pub dir: Option<String>,
    pub tabindex: Option<i32>,
    pub accesskey: Option<String>,
    pub hidden: Option<bool>,
    pub draggable: Option<String>,
    pub translate: Option<String>,
    pub contenteditable: Option<String>,
    pub spellcheck: Option<bool>,
    pub data_attrs: HashMap<String, String>,
}

/// WAI-ARIA 1.2 Attributes
#[derive(Default, Debug, Clone)]
pub struct AriaAttrs {
    pub aria_label: Option<String>,
    pub aria_labelledby: Option<String>,
    pub aria_describedby: Option<String>,
    pub aria_hidden: Option<bool>,
    pub aria_disabled: Option<bool>,
    pub aria_expanded: Option<bool>,
    pub aria_selected: Option<bool>,
    pub aria_checked: Option<bool>,
    pub aria_invalid: Option<bool>,
    pub aria_required: Option<bool>,
    pub aria_owns: Option<String>,
    pub aria_controls: Option<String>,
    pub aria_current: Option<String>,
    pub aria_details: Option<String>,
    pub aria_errormessage: Option<String>,
    pub aria_flowto: Option<String>,
    pub aria_keyshortcuts: Option<String>,
    pub aria_live: Option<String>,
    pub aria_relevant: Option<String>,
    pub aria_atomic: Option<bool>,
    pub aria_colcount: Option<i32>,
    pub aria_colindex: Option<i32>,
    pub aria_colspan: Option<i32>,
    pub aria_rowcount: Option<i32>,
    pub aria_rowindex: Option<i32>,
    pub aria_rowspan: Option<i32>,
    pub aria_sort: Option<String>,
    pub aria_description: Option<String>,
    pub aria_busy: Option<bool>,
}

/// HTMX 2.0 Attributes
#[derive(Default, Debug, Clone)]
pub struct HtmxAttrs {
    pub hx_get: Option<String>,
    pub hx_post: Option<String>,
    pub hx_put: Option<String>,
    pub hx_delete: Option<String>,
    pub hx_patch: Option<String>,
    pub hx_trigger: Option<String>,
    pub hx_target: Option<String>,
    pub hx_swap: Option<String>,
    pub hx_swap_oob: Option<bool>,
    pub hx_indicator: Option<String>,
    pub hx_headers: Option<String>,
    pub hx_ext: Option<String>,
    pub hx_boost: Option<bool>,
    pub hx_history: Option<bool>,
    pub hx_include: Option<String>,
    pub hx_exclude: Option<String>,
    pub hx_replace_url: Option<String>,
    pub hx_preserve: Option<bool>,
    pub hx_prompt: Option<String>,
    pub hx_confirm: Option<String>,
    pub hx_on: Option<String>,
    pub hx_params: Option<String>,
    pub hx_validate: Option<bool>,
}

/// Alpine.js Attributes
#[derive(Default, Debug, Clone)]
pub struct AlpineAttrs {
    pub x_data: Option<String>,
    pub x_init: Option<String>,
    pub x_bind: Option<String>,
    pub x_on: Option<String>,
    pub x_model: Option<String>,
    pub x_show: Option<String>,
    pub x_for: Option<String>,
    pub x_transition: Option<String>,
    pub x_effect: Option<String>,
    pub x_cloak: Option<bool>,
    pub x_ignore: Option<bool>,
    pub x_text: Option<String>,
    pub x_html: Option<String>,
    pub x_ref: Option<String>,
    pub x_mask: Option<String>,
    pub x_scope: Option<String>,
    pub x_teleport: Option<String>,
}

/// DOM Event Handlers
#[derive(Default, Debug, Clone)]
pub struct EventAttrs {
    pub onclick: Option<String>,
    pub ondblclick: Option<String>,
    pub oncontextmenu: Option<String>,
    pub onmousedown: Option<String>,
    pub onmouseup: Option<String>,
    pub onmouseenter: Option<String>,
    pub onmouseleave: Option<String>,
    pub onmousemove: Option<String>,
    pub onmouseover: Option<String>,
    pub onmouseout: Option<String>,
    pub onkeydown: Option<String>,
    pub onkeyup: Option<String>,
    pub onkeypress: Option<String>,
    pub onfocus: Option<String>,
    pub onblur: Option<String>,
    pub onfocusin: Option<String>,
    pub onfocusout: Option<String>,
    pub onchange: Option<String>,
    pub oninput: Option<String>,
    pub onsubmit: Option<String>,
    pub onreset: Option<String>,
    pub oninvalid: Option<String>,
    pub oncut: Option<String>,
    pub oncopy: Option<String>,
    pub onpaste: Option<String>,
    pub onload: Option<String>,
    pub onerror: Option<String>,
    pub onscroll: Option<String>,
}

/// Combined attributes for `#[ui]` macro
#[derive(Default, Debug, Clone)]
pub struct AllAttrs {
    pub html: HtmlAttrs,
    pub aria: AriaAttrs,
    pub htmx: HtmxAttrs,
    pub alpine: AlpineAttrs,
    pub events: EventAttrs,
}

/// Trait for rendering attributes to HTML string
pub trait RenderAttrs {
    fn render_attrs(&self) -> String;
}

impl RenderAttrs for HtmlAttrs {
    fn render_attrs(&self) -> String {
        let mut parts = Vec::new();

        if let Some(ref v) = self.id {
            parts.push(format!(r#" id="{}""#, v));
        }
        if let Some(ref v) = self.name {
            parts.push(format!(r#" name="{}""#, v));
        }
        if let Some(ref v) = self.class {
            parts.push(format!(r#" class="{}""#, v));
        }
        if let Some(ref v) = self.style {
            parts.push(format!(r#" style="{}""#, v));
        }
        if let Some(ref v) = self.title {
            parts.push(format!(r#" title="{}""#, v));
        }
        if let Some(ref v) = self.lang {
            parts.push(format!(r#" lang="{}""#, v));
        }
        if let Some(ref v) = self.dir {
            parts.push(format!(r#" dir="{}""#, v));
        }
        if let Some(v) = self.tabindex {
            parts.push(format!(r#" tabindex="{}""#, v));
        }
        if let Some(ref v) = self.accesskey {
            parts.push(format!(r#" accesskey="{}""#, v));
        }
        if self.hidden == Some(true) {
            parts.push(" hidden".to_string());
        }
        if let Some(ref v) = self.draggable {
            parts.push(format!(r#" draggable="{}""#, v));
        }
        if let Some(ref v) = self.translate {
            parts.push(format!(r#" translate="{}""#, v));
        }
        if let Some(ref v) = self.contenteditable {
            parts.push(format!(r#" contenteditable="{}""#, v));
        }
        if let Some(v) = self.spellcheck {
            parts.push(format!(r#" spellcheck="{}""#, v));
        }
        for (key, value) in &self.data_attrs {
            parts.push(format!(r#" data-{}="{}""#, key, value));
        }

        parts.join("")
    }
}

impl RenderAttrs for AriaAttrs {
    fn render_attrs(&self) -> String {
        let mut parts = Vec::new();

        if let Some(ref v) = self.aria_label {
            parts.push(format!(r#" aria-label="{}""#, v));
        }
        if let Some(ref v) = self.aria_labelledby {
            parts.push(format!(r#" aria-labelledby="{}""#, v));
        }
        if let Some(ref v) = self.aria_describedby {
            parts.push(format!(r#" aria-describedby="{}""#, v));
        }
        if let Some(v) = self.aria_hidden {
            if v {
                parts.push(r#" aria-hidden="true""#.to_string());
            }
        }
        if let Some(v) = self.aria_disabled {
            if v {
                parts.push(r#" aria-disabled="true""#.to_string());
            }
        }
        if let Some(v) = self.aria_expanded {
            if v {
                parts.push(r#" aria-expanded="true""#.to_string());
            }
        }
        if let Some(v) = self.aria_selected {
            if v {
                parts.push(r#" aria-selected="true""#.to_string());
            }
        }
        if let Some(v) = self.aria_checked {
            if v {
                parts.push(r#" aria-checked="true""#.to_string());
            }
        }
        if let Some(v) = self.aria_invalid {
            if v {
                parts.push(r#" aria-invalid="true""#.to_string());
            }
        }
        if let Some(v) = self.aria_required {
            if v {
                parts.push(r#" aria-required="true""#.to_string());
            }
        }
        if let Some(ref v) = self.aria_owns {
            parts.push(format!(r#" aria-owns="{}""#, v));
        }
        if let Some(ref v) = self.aria_controls {
            parts.push(format!(r#" aria-controls="{}""#, v));
        }
        if let Some(ref v) = self.aria_current {
            parts.push(format!(r#" aria-current="{}""#, v));
        }
        if let Some(ref v) = self.aria_details {
            parts.push(format!(r#" aria-details="{}""#, v));
        }
        if let Some(ref v) = self.aria_errormessage {
            parts.push(format!(r#" aria-errormessage="{}""#, v));
        }
        if let Some(ref v) = self.aria_flowto {
            parts.push(format!(r#" aria-flowto="{}""#, v));
        }
        if let Some(ref v) = self.aria_keyshortcuts {
            parts.push(format!(r#" aria-keyshortcuts="{}""#, v));
        }
        if let Some(ref v) = self.aria_live {
            parts.push(format!(r#" aria-live="{}""#, v));
        }
        if let Some(ref v) = self.aria_relevant {
            parts.push(format!(r#" aria-relevant="{}""#, v));
        }
        if let Some(v) = self.aria_atomic {
            if v {
                parts.push(r#" aria-atomic="true""#.to_string());
            }
        }
        if let Some(v) = self.aria_colcount {
            parts.push(format!(r#" aria-colcount="{}""#, v));
        }
        if let Some(v) = self.aria_colindex {
            parts.push(format!(r#" aria-colindex="{}""#, v));
        }
        if let Some(v) = self.aria_colspan {
            parts.push(format!(r#" aria-colspan="{}""#, v));
        }
        if let Some(v) = self.aria_rowcount {
            parts.push(format!(r#" aria-rowcount="{}""#, v));
        }
        if let Some(v) = self.aria_rowindex {
            parts.push(format!(r#" aria-rowindex="{}""#, v));
        }
        if let Some(v) = self.aria_rowspan {
            parts.push(format!(r#" aria-rowspan="{}""#, v));
        }
        if let Some(ref v) = self.aria_sort {
            parts.push(format!(r#" aria-sort="{}""#, v));
        }
        if let Some(ref v) = self.aria_description {
            parts.push(format!(r#" aria-description="{}""#, v));
        }
        if let Some(v) = self.aria_busy {
            if v {
                parts.push(r#" aria-busy="true""#.to_string());
            }
        }

        parts.join("")
    }
}

impl RenderAttrs for HtmxAttrs {
    fn render_attrs(&self) -> String {
        let mut parts = Vec::new();

        if let Some(ref v) = self.hx_get {
            parts.push(format!(r#" hx-get="{}""#, v));
        }
        if let Some(ref v) = self.hx_post {
            parts.push(format!(r#" hx-post="{}""#, v));
        }
        if let Some(ref v) = self.hx_put {
            parts.push(format!(r#" hx-put="{}""#, v));
        }
        if let Some(ref v) = self.hx_delete {
            parts.push(format!(r#" hx-delete="{}""#, v));
        }
        if let Some(ref v) = self.hx_patch {
            parts.push(format!(r#" hx-patch="{}""#, v));
        }
        if let Some(ref v) = self.hx_trigger {
            parts.push(format!(r#" hx-trigger="{}""#, v));
        }
        if let Some(ref v) = self.hx_target {
            parts.push(format!(r#" hx-target="{}""#, v));
        }
        if let Some(ref v) = self.hx_swap {
            parts.push(format!(r#" hx-swap="{}""#, v));
        }
        if let Some(v) = self.hx_swap_oob {
            if v {
                parts.push(r#" hx-swap-oob="true""#.to_string());
            }
        }
        if let Some(ref v) = self.hx_indicator {
            parts.push(format!(r#" hx-indicator="{}""#, v));
        }
        if let Some(ref v) = self.hx_headers {
            parts.push(format!(r#" hx-headers="{}""#, v));
        }
        if let Some(ref v) = self.hx_ext {
            parts.push(format!(r#" hx-ext="{}""#, v));
        }
        if let Some(v) = self.hx_boost {
            if v {
                parts.push(r#" hx-boost="true""#.to_string());
            }
        }
        if let Some(v) = self.hx_history {
            if v {
                parts.push(r#" hx-history="true""#.to_string());
            }
        }
        if let Some(ref v) = self.hx_include {
            parts.push(format!(r#" hx-include="{}""#, v));
        }
        if let Some(ref v) = self.hx_exclude {
            parts.push(format!(r#" hx-exclude="{}""#, v));
        }
        if let Some(ref v) = self.hx_replace_url {
            parts.push(format!(r#" hx-replace-url="{}""#, v));
        }
        if let Some(v) = self.hx_preserve {
            if v {
                parts.push(r#" hx-preserve="true""#.to_string());
            }
        }
        if let Some(ref v) = self.hx_prompt {
            parts.push(format!(r#" hx-prompt="{}""#, v));
        }
        if let Some(ref v) = self.hx_confirm {
            parts.push(format!(r#" hx-confirm="{}""#, v));
        }
        if let Some(ref v) = self.hx_on {
            parts.push(format!(r#" hx-on="{}""#, v));
        }
        if let Some(ref v) = self.hx_params {
            parts.push(format!(r#" hx-params="{}""#, v));
        }
        if let Some(v) = self.hx_validate {
            if v {
                parts.push(r#" hx-validate="true""#.to_string());
            }
        }

        parts.join("")
    }
}

impl RenderAttrs for AlpineAttrs {
    fn render_attrs(&self) -> String {
        let mut parts = Vec::new();

        if let Some(ref v) = self.x_data {
            parts.push(format!(r#" x-data="{}""#, v));
        }
        if let Some(ref v) = self.x_init {
            parts.push(format!(r#" x-init="{}""#, v));
        }
        if let Some(ref v) = self.x_bind {
            parts.push(format!(r#" x-bind="{}""#, v));
        }
        if let Some(ref v) = self.x_on {
            parts.push(format!(r#" x-on="{}""#, v));
        }
        if let Some(ref v) = self.x_model {
            parts.push(format!(r#" x-model="{}""#, v));
        }
        if let Some(ref v) = self.x_show {
            parts.push(format!(r#" x-show="{}""#, v));
        }
        if let Some(ref v) = self.x_for {
            parts.push(format!(r#" x-for="{}""#, v));
        }
        if let Some(ref v) = self.x_transition {
            parts.push(format!(r#" x-transition="{}""#, v));
        }
        if let Some(ref v) = self.x_effect {
            parts.push(format!(r#" x-effect="{}""#, v));
        }
        if let Some(v) = self.x_cloak {
            if v {
                parts.push(r#" x-cloak="true""#.to_string());
            }
        }
        if let Some(v) = self.x_ignore {
            if v {
                parts.push(r#" x-ignore="true""#.to_string());
            }
        }
        if let Some(ref v) = self.x_text {
            parts.push(format!(r#" x-text="{}""#, v));
        }
        if let Some(ref v) = self.x_html {
            parts.push(format!(r#" x-html="{}""#, v));
        }
        if let Some(ref v) = self.x_ref {
            parts.push(format!(r#" x-ref="{}""#, v));
        }
        if let Some(ref v) = self.x_mask {
            parts.push(format!(r#" x-mask="{}""#, v));
        }
        if let Some(ref v) = self.x_scope {
            parts.push(format!(r#" x-scope="{}""#, v));
        }
        if let Some(ref v) = self.x_teleport {
            parts.push(format!(r#" x-teleport="{}""#, v));
        }

        parts.join("")
    }
}

impl RenderAttrs for EventAttrs {
    fn render_attrs(&self) -> String {
        let mut parts = Vec::new();

        if let Some(ref v) = self.onclick {
            parts.push(format!(r#" onclick="{}""#, v));
        }
        if let Some(ref v) = self.ondblclick {
            parts.push(format!(r#" ondblclick="{}""#, v));
        }
        if let Some(ref v) = self.oncontextmenu {
            parts.push(format!(r#" oncontextmenu="{}""#, v));
        }
        if let Some(ref v) = self.onmousedown {
            parts.push(format!(r#" onmousedown="{}""#, v));
        }
        if let Some(ref v) = self.onmouseup {
            parts.push(format!(r#" onmouseup="{}""#, v));
        }
        if let Some(ref v) = self.onmouseenter {
            parts.push(format!(r#" onmouseenter="{}""#, v));
        }
        if let Some(ref v) = self.onmouseleave {
            parts.push(format!(r#" onmouseleave="{}""#, v));
        }
        if let Some(ref v) = self.onmousemove {
            parts.push(format!(r#" onmousemove="{}""#, v));
        }
        if let Some(ref v) = self.onmouseover {
            parts.push(format!(r#" onmouseover="{}""#, v));
        }
        if let Some(ref v) = self.onmouseout {
            parts.push(format!(r#" onmouseout="{}""#, v));
        }
        if let Some(ref v) = self.onkeydown {
            parts.push(format!(r#" onkeydown="{}""#, v));
        }
        if let Some(ref v) = self.onkeyup {
            parts.push(format!(r#" onkeyup="{}""#, v));
        }
        if let Some(ref v) = self.onkeypress {
            parts.push(format!(r#" onkeypress="{}""#, v));
        }
        if let Some(ref v) = self.onfocus {
            parts.push(format!(r#" onfocus="{}""#, v));
        }
        if let Some(ref v) = self.onblur {
            parts.push(format!(r#" onblur="{}""#, v));
        }
        if let Some(ref v) = self.onfocusin {
            parts.push(format!(r#" onfocusin="{}""#, v));
        }
        if let Some(ref v) = self.onfocusout {
            parts.push(format!(r#" onfocusout="{}""#, v));
        }
        if let Some(ref v) = self.onchange {
            parts.push(format!(r#" onchange="{}""#, v));
        }
        if let Some(ref v) = self.oninput {
            parts.push(format!(r#" oninput="{}""#, v));
        }
        if let Some(ref v) = self.onsubmit {
            parts.push(format!(r#" onsubmit="{}""#, v));
        }
        if let Some(ref v) = self.onreset {
            parts.push(format!(r#" onreset="{}""#, v));
        }
        if let Some(ref v) = self.oninvalid {
            parts.push(format!(r#" oninvalid="{}""#, v));
        }
        if let Some(ref v) = self.oncopy {
            parts.push(format!(r#" oncopy="{}""#, v));
        }
        if let Some(ref v) = self.oncut {
            parts.push(format!(r#" oncut="{}""#, v));
        }
        if let Some(ref v) = self.onpaste {
            parts.push(format!(r#" onpaste="{}""#, v));
        }
        if let Some(ref v) = self.onload {
            parts.push(format!(r#" onload="{}""#, v));
        }
        if let Some(ref v) = self.onerror {
            parts.push(format!(r#" onerror="{}""#, v));
        }
        if let Some(ref v) = self.onscroll {
            parts.push(format!(r#" onscroll="{}""#, v));
        }

        parts.join("")
    }
}

impl RenderAttrs for AllAttrs {
    fn render_attrs(&self) -> String {
        let mut parts = Vec::new();

        let html_attrs = RenderAttrs::render_attrs(&self.html);
        if !html_attrs.is_empty() {
            parts.push(html_attrs);
        }

        let aria_attrs = RenderAttrs::render_attrs(&self.aria);
        if !aria_attrs.is_empty() {
            parts.push(aria_attrs);
        }

        let htmx_attrs = RenderAttrs::render_attrs(&self.htmx);
        if !htmx_attrs.is_empty() {
            parts.push(htmx_attrs);
        }

        let alpine_attrs = RenderAttrs::render_attrs(&self.alpine);
        if !alpine_attrs.is_empty() {
            parts.push(alpine_attrs);
        }

        let events_attrs = RenderAttrs::render_attrs(&self.events);
        if !events_attrs.is_empty() {
            parts.push(events_attrs);
        }

        parts.join("")
    }
}

impl AllAttrs {
    pub fn new() -> Self {
        Self::default()
    }
}
