use axum::response::Html;
use rsx::rsx;
use rsxui::components::{Modal, ModalAction, ModalBox, ModalPlacement, ModalToggle};

use crate::layout;

pub async fn page() -> Html<String> {
    // ========================================================
    // Method 1: HTML dialog element (recommended)
    // ========================================================

    let dialog_basic = rsx! {
        <button class="btn" onclick="document.getElementById('my_modal_1').showModal()">
            "open modal"
        </button>
        <Modal id="my_modal_1" placement=ModalPlacement::Middle>
            <ModalBox>
                <h3 class="font-bold text-lg">"Hello!"</h3>
                <p class="py-4">"Press ESC key or click the button below to close"</p>
                <ModalAction>
                    <form method="dialog">
                        <button class="btn">"Close"</button>
                    </form>
                </ModalAction>
            </ModalBox>
        </Modal>
    };

    let dialog_backdrop = rsx! {
        <button class="btn" onclick="document.getElementById('my_modal_2').showModal()">
            "open modal"
        </button>
        <Modal id="my_modal_2" placement=ModalPlacement::Middle>
            <ModalBox>
                <h3 class="font-bold text-lg">"Click Outside"</h3>
                <p class="py-4">"Press ESC key or click outside to close"</p>
            </ModalBox>
            <form method="dialog" class="modal-backdrop">
                <button>"close"</button>
            </form>
        </Modal>
    };

    let dialog_corner_close = rsx! {
        <button class="btn" onclick="document.getElementById('my_modal_3').showModal()">
            "open modal"
        </button>
        <Modal id="my_modal_3" placement=ModalPlacement::Middle>
            <ModalBox>
                <form method="dialog">
                    <button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">
                        "✕"
                    </button>
                </form>
                <h3 class="font-bold text-lg">"Corner Close"</h3>
                <p class="py-4">"Press ESC key or click on ✕ button to close"</p>
            </ModalBox>
        </Modal>
    };

    let dialog_custom_width = rsx! {
        <button class="btn" onclick="document.getElementById('my_modal_4').showModal()">
            "open modal"
        </button>
        <Modal id="my_modal_4" placement=ModalPlacement::Middle>
            <ModalBox class="w-11/12 max-w-5xl">
                <h3 class="font-bold text-lg">"Custom Width"</h3>
                <p class="py-4">"This modal uses custom width classes"</p>
                <ModalAction>
                    <form method="dialog">
                        <button class="btn">"Close"</button>
                    </form>
                </ModalAction>
            </ModalBox>
        </Modal>
    };

    let dialog_responsive = rsx! {
        <button class="btn" onclick="document.getElementById('my_modal_5').showModal()">
            "open modal"
        </button>
        <Modal id="my_modal_5" class="modal-bottom sm:modal-middle">
            <ModalBox>
                <h3 class="font-bold text-lg">"Responsive"</h3>
                <p class="py-4">"Bottom on small screens, middle on larger screens"</p>
                <ModalAction>
                    <form method="dialog">
                        <button class="btn">"Close"</button>
                    </form>
                </ModalAction>
            </ModalBox>
        </Modal>
    };

    let dialog_top = rsx! {
        <button class="btn" onclick="document.getElementById('modal_top').showModal()">
            "top"
        </button>
        <Modal id="modal_top" placement=ModalPlacement::Top>
            <ModalBox>
                <h3 class="font-bold text-lg">"Top Placement"</h3>
                <p class="py-4">"This modal appears at the top"</p>
                <ModalAction>
                    <form method="dialog">
                        <button class="btn">"Close"</button>
                    </form>
                </ModalAction>
            </ModalBox>
        </Modal>
    };

    let dialog_bottom = rsx! {
        <button class="btn" onclick="document.getElementById('modal_bottom').showModal()">
            "bottom"
        </button>
        <Modal id="modal_bottom" placement=ModalPlacement::Bottom>
            <ModalBox>
                <h3 class="font-bold text-lg">"Bottom Placement"</h3>
                <p class="py-4">"This modal appears at the bottom"</p>
                <ModalAction>
                    <form method="dialog">
                        <button class="btn">"Close"</button>
                    </form>
                </ModalAction>
            </ModalBox>
        </Modal>
    };

    let dialog_start = rsx! {
        <button class="btn" onclick="document.getElementById('modal_start').showModal()">
            "start"
        </button>
        <Modal id="modal_start" placement=ModalPlacement::Start>
            <ModalBox>
                <h3 class="font-bold text-lg">"Start Placement"</h3>
                <p class="py-4">"This modal appears at the start (left)"</p>
                <ModalAction>
                    <form method="dialog">
                        <button class="btn">"Close"</button>
                    </form>
                </ModalAction>
            </ModalBox>
        </Modal>
    };

    let dialog_end = rsx! {
        <button class="btn" onclick="document.getElementById('modal_end').showModal()">
            "end"
        </button>
        <Modal id="modal_end" placement=ModalPlacement::End>
            <ModalBox>
                <h3 class="font-bold text-lg">"End Placement"</h3>
                <p class="py-4">"This modal appears at the end (right)"</p>
                <ModalAction>
                    <form method="dialog">
                        <button class="btn">"Close"</button>
                    </form>
                </ModalAction>
            </ModalBox>
        </Modal>
    };

    // ========================================================
    // Method 2: Checkbox (legacy)
    // ========================================================

    let checkbox_basic = rsx! {
        <label for="my_modal_6" class="btn">"open modal"</label>
        <ModalToggle id="my_modal_6" />
        <div class="modal" role="dialog">
            <ModalBox>
                <h3 class="font-bold text-lg">"Checkbox Modal"</h3>
                <p class="py-4">"This modal works with a hidden checkbox!"</p>
                <ModalAction>
                    <label for="my_modal_6" class="btn">"Close!"</label>
                </ModalAction>
            </ModalBox>
        </div>
    };

    let checkbox_backdrop = rsx! {
        <label for="my_modal_7" class="btn">"open modal"</label>
        <ModalToggle id="my_modal_7" />
        <div class="modal" role="dialog">
            <ModalBox>
                <h3 class="font-bold text-lg">"Checkbox + Backdrop"</h3>
                <p class="py-4">"Click outside to close using modal-backdrop"</p>
            </ModalBox>
            <label class="modal-backdrop" for="my_modal_7">"Close"</label>
        </div>
    };

    // ========================================================
    // Method 3: Anchor links (legacy)
    // ========================================================

    let anchor_basic = rsx! {
        <a href="#my_modal_8" class="btn">"open modal"</a>
        <div class="modal" role="dialog" id="my_modal_8">
            <ModalBox>
                <h3 class="font-bold text-lg">"Anchor Modal"</h3>
                <p class="py-4">"This modal works with anchor links"</p>
                <ModalAction>
                    <a href="#" class="btn">"Yay!"</a>
                </ModalAction>
            </ModalBox>
        </div>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Modal"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Modal is used to show a dialog or a box when you click a button."
            </p>

            <div class="alert alert-info mb-8">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="h-6 w-6 shrink-0 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                <span>"There are 3 methods: HTML dialog (recommended), checkbox (legacy), and anchor links (legacy)."</span>
            </div>

            // ========================================================
            // Method 1: Dialog
            // ========================================================

            <h2 class="text-2xl font-bold mt-12 mb-4">"Method 1: HTML dialog element (recommended)"</h2>
            <p class="text-base-content/70 mb-6">
                "Open with JS "<code>"document.getElementById('ID').showModal()"</code>". Close with ESC key or dialog button."
            </p>

            <div class="divider">"Dialog Modal"</div>
            <p class="text-sm text-gray-600 mb-4">"Basic modal opened with showModal()."</p>
            <div class="mb-4">{dialog_basic}</div>

            <div class="divider">"Close on Backdrop Click"</div>
            <p class="text-sm text-gray-600 mb-4">"Add a second form with modal-backdrop class."</p>
            <div class="mb-4">{dialog_backdrop}</div>

            <div class="divider">"Close Button at Corner"</div>
            <p class="text-sm text-gray-600 mb-4">"Place a form with btn-circle button inside modal-box."</p>
            <div class="mb-4">{dialog_corner_close}</div>

            <div class="divider">"Custom Width"</div>
            <p class="text-sm text-gray-600 mb-4">"Use any w-* and max-w-* utility class on modal-box."</p>
            <div class="mb-4">{dialog_custom_width}</div>

            <div class="divider">"Responsive"</div>
            <p class="text-sm text-gray-600 mb-4">"Bottom on SM, middle on MD using responsive prefixes."</p>
            <div class="mb-4">{dialog_responsive}</div>

            <div class="divider">"Placements"</div>
            <p class="text-sm text-gray-600 mb-4">"Top, bottom, start, end positions."</p>
            <div class="flex flex-wrap gap-2 mb-4">
                {dialog_top}
                {dialog_bottom}
                {dialog_start}
                {dialog_end}
            </div>

            // ========================================================
            // Method 2: Checkbox
            // ========================================================

            <h2 class="text-2xl font-bold mt-12 mb-4">"Method 2: Checkbox (legacy)"</h2>
            <p class="text-base-content/70 mb-6">
                "A hidden checkbox controls the state. Labels toggle the checkbox to open/close."
            </p>

            <div class="divider">"Checkbox Modal"</div>
            <p class="text-sm text-gray-600 mb-4">"Toggle using a hidden checkbox and label."</p>
            <div class="mb-4">{checkbox_basic}</div>

            <div class="divider">"Checkbox + Backdrop Close"</div>
            <p class="text-sm text-gray-600 mb-4">"Use modal-backdrop label to close when clicked outside."</p>
            <div class="mb-4">{checkbox_backdrop}</div>

            // ========================================================
            // Method 3: Anchor
            // ========================================================

            <h2 class="text-2xl font-bold mt-12 mb-4">"Method 3: Anchor links (legacy)"</h2>
            <p class="text-base-content/70 mb-6">
                "A link adds a URL hash. The modal is visible only when the URL contains that hash."
            </p>

            <div class="divider">"Anchor Modal"</div>
            <p class="text-sm text-gray-600 mb-4">"Open/close using anchor links."</p>
            <div class="mb-4">{anchor_basic}</div>

            // ========================================================
            // Code Example
            // ========================================================

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Modal, ModalBox, ModalAction, ModalPlacement};",
                "use rsx::rsx;",
                "",
                "// Trigger button + dialog modal",
                "let html = rsx! {",
                "    <button onclick=\"document.getElementById('my_modal').showModal()\">",
                "        \"open modal\"",
                "    </button>",
                "    <Modal id=\"my_modal\" placement=ModalPlacement::Middle>",
                "        <ModalBox>",
                "            <h3>\"Hello!\"</h3>",
                "            <ModalAction>",
                "                <form method=\"dialog\">",
                "                    <button class=\"btn\">\"Close\"</button>",
                "                </form>",
                "            </ModalAction>",
                "        </ModalBox>",
                "    </Modal>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/modal").await)
}
