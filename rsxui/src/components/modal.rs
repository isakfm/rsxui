//! Modal Component
//!
//! Modal is used to show a dialog or a box when you click a button.
//!
//! # DaisyUI Classes
//! - Base: `modal`
//! - Part: `modal-box`, `modal-action`, `modal-backdrop`, `modal-toggle`
//! - Modifier: `modal-open`
//! - Placement: `modal-top`, `modal-middle`, `modal-bottom`, `modal-start`, `modal-end`
//!
//! # Example
//!
//! ```rust,ignore
//! use rsxui::components::{Modal, ModalBox, ModalAction, ModalPlacement};
//! use rsx::rsx;
//!
//! let html = rsx! {
//!     <Modal placement={ModalPlacement::Middle} id="my_modal">
//!         <ModalBox>
//!             <h3 class="text-lg font-bold">"Hello!"</h3>
//!             <ModalAction>
//!                 <form method="dialog"><button class="btn">"Close"</button></form>
//!             </ModalAction>
//!         </ModalBox>
//!     </Modal>
//! };
//! ```

use enum_stringify::EnumStringify;
use rsx::attrs::RenderAttrs;
use rsx::{classes, component, rsx, ui};

use crate::components::{class_if, show_if};

// ============================================
// ModalPlacement - Modal position on screen
// ============================================

#[derive(Debug, Clone, PartialEq, Default, EnumStringify)]
#[enum_stringify(case = "kebab", prefix = "modal-")]
pub enum ModalPlacement {
    #[default]
    #[enum_stringify(rename = "")]
    Default,
    Top,
    Middle,
    Bottom,
    Start,
    End,
}

// ============================================
// ModalBox - Content container inside modal
// ============================================

#[component]
pub fn ModalBox(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div class={classes!("modal-box", class)}>{children}</div>
    }
}

// ============================================
// ModalAction - Action buttons area inside modal
// ============================================

#[component]
pub fn ModalAction(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div class={classes!("modal-action", class)}>{children}</div>
    }
}

// ============================================
// ModalBackdrop - Click-to-close overlay
// ============================================

#[ui]
pub fn ModalBackdrop(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <label
            class={classes!("modal-backdrop", props.class)}
            {props.render_attrs()}>{props.children}</label>
    }
}

// ============================================
// ModalToggle - Hidden checkbox for legacy modal
// ============================================

#[component]
pub fn ModalToggle(
    id: String,
    #[builder(default)] class: String,
    #[builder(default)] checked: bool,
) -> String {
    rsx! {
        <input type="checkbox" id={id} class={classes!("modal-toggle", class)} {show_if(checked, "checked")} />
    }
}

// ============================================
// Modal - Modal container
// ============================================

#[ui]
pub fn Modal(
    #[builder(default)] placement: ModalPlacement,
    #[builder(default)] open: bool,
    #[builder(default)] class: String,
    children: String,
) -> String {
    rsx! {
        <dialog
            class={classes!("modal", props.placement, class_if(props.open, "modal-open"), props.class)}
            {show_if(props.open, "open")}
            {props.render_attrs()}>{props.children}</dialog>
    }
}

// ============================================
// Tests
// ============================================

#[cfg(test)]
mod tests {
    use super::*;
    use rsx::rsx;

    #[tokio::test]
    async fn test_modal_basic() {
        let html = rsx! {
            <Modal>
                <ModalBox>"Content"</ModalBox>
            </Modal>
        };
        assert!(html.contains("<dialog"));
        assert!(html.contains(r#"class="modal""#));
        assert!(html.contains("modal-box"));
        assert!(html.contains(">Content</div>"));
    }

    #[tokio::test]
    async fn test_modal_with_id() {
        let html = rsx! {
            <Modal id="my_modal">
                <ModalBox>"Content"</ModalBox>
            </Modal>
        };
        assert!(html.contains(r#"id="my_modal""#));
    }

    #[tokio::test]
    async fn test_modal_with_onclick() {
        let html = rsx! {
            <Modal onclick="my_modal.showModal()">
                <ModalBox>"Content"</ModalBox>
            </Modal>
        };
        assert!(html.contains(r#"onclick="my_modal.showModal()""#));
    }

    #[tokio::test]
    async fn test_modal_all_placements() {
        for placement in [
            ModalPlacement::Top,
            ModalPlacement::Middle,
            ModalPlacement::Bottom,
            ModalPlacement::Start,
            ModalPlacement::End,
        ] {
            let html = rsx! {
                <Modal placement=placement.clone()>
                    <ModalBox>"X"</ModalBox>
                </Modal>
            };
            let expected = placement.to_string();
            assert!(
                html.contains(&expected),
                "Missing placement class {} for {:?}",
                expected,
                placement
            );
        }
    }

    #[tokio::test]
    async fn test_modal_open() {
        let html = rsx! {
            <Modal open=true>
                <ModalBox>"Content"</ModalBox>
            </Modal>
        };
        assert!(html.contains("modal-open"));
        assert!(html.contains(" open"));
    }

    #[tokio::test]
    async fn test_modal_action() {
        let html = rsx! {
            <ModalAction>
                <button class="btn">"Close"</button>
            </ModalAction>
        };
        assert!(html.contains(r#"class="modal-action""#));
        assert!(html.contains(">Close</button>"));
    }

    #[tokio::test]
    async fn test_modal_backdrop() {
        let html = rsx! {
            <ModalBackdrop>"Close"</ModalBackdrop>
        };
        assert!(html.contains(r#"class="modal-backdrop""#));
        assert!(html.contains(">Close</label>"));
    }

    #[tokio::test]
    async fn test_modal_backdrop_custom_class() {
        let html = rsx! {
            <ModalBackdrop class="z-50">"Close"</ModalBackdrop>
        };
        assert!(html.contains("modal-backdrop z-50"));
    }

    #[tokio::test]
    async fn test_modal_backdrop_without_for() {
        let html = rsx! {
            <ModalBackdrop>"Close"</ModalBackdrop>
        };
        assert!(html.contains(r#"class="modal-backdrop""#));
        assert!(!html.contains("for="));
    }

    #[tokio::test]
    async fn test_modal_toggle() {
        let html = rsx! {
            <ModalToggle id="my_modal_toggle" />
        };
        assert!(html.contains(r#"type="checkbox""#));
        assert!(html.contains(r#"id="my_modal_toggle""#));
        assert!(html.contains("modal-toggle"));
    }

    #[tokio::test]
    async fn test_modal_toggle_checked() {
        let html = rsx! {
            <ModalToggle id="my_toggle" checked=true />
        };
        assert!(html.contains(" checked"));
    }

    #[tokio::test]
    async fn test_modal_custom_class() {
        let html = rsx! {
            <Modal class="modal-bottom sm:modal-middle">
                <ModalBox class="w-11/12 max-w-5xl">"Content"</ModalBox>
            </Modal>
        };
        assert!(html.contains("modal modal-bottom sm:modal-middle"));
        assert!(html.contains("modal-box w-11/12 max-w-5xl"));
    }

    #[tokio::test]
    async fn test_modal_default_no_extra_classes() {
        let html = rsx! {
            <Modal>
                <ModalBox>"X"</ModalBox>
            </Modal>
        };
        assert!(!html.contains("modal-middle"));
        assert!(!html.contains("modal-open"));
    }

    #[tokio::test]
    async fn test_modal_composition() {
        let html = rsx! {
            <Modal id="my_modal" placement=ModalPlacement::Middle>
                <ModalBox>
                    <h3>"Title"</h3>
                    <p>"Body"</p>
                    <ModalAction>
                        <form method="dialog"><button>"Close"</button></form>
                    </ModalAction>
                </ModalBox>
                <form method="dialog" class="modal-backdrop"><button>"close"</button></form>
            </Modal>
        };
        assert!(html.contains("id=\"my_modal\""));
        assert!(html.contains("modal-middle"));
        assert!(html.contains("modal-box"));
        assert!(html.contains("modal-action"));
        assert!(html.contains("modal-backdrop"));
    }
}
