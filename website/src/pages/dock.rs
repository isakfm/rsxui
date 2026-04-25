use axum::response::Html;
use rsx_macros::rsx;
use rsxui::components::{Dock, DockItem, Size};

use crate::layout;

fn home_icon() -> &'static str {
    r#"<svg class="size-[1.2em]" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g fill="currentColor" stroke-linejoin="miter" stroke-linecap="butt"><polyline points="1 11 12 2 23 11" fill="none" stroke="currentColor" stroke-miterlimit="10" stroke-width="2"></polyline><path d="m5,13v7c0,1.105.895,2,2,2h10c1.105,0,2-.895,2-2v-7" fill="none" stroke="currentColor" stroke-linecap="square" stroke-miterlimit="10" stroke-width="2"></path><line x1="12" y1="22" x2="12" y2="18" fill="none" stroke="currentColor" stroke-linecap="square" stroke-miterlimit="10" stroke-width="2"></line></g></svg>"#
}

fn inbox_icon() -> &'static str {
    r#"<svg class="size-[1.2em]" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g fill="currentColor" stroke-linejoin="miter" stroke-linecap="butt"><polyline points="3 14 9 14 9 17 15 17 15 14 21 14" fill="none" stroke="currentColor" stroke-miterlimit="10" stroke-width="2"></polyline><rect x="3" y="3" width="18" height="18" rx="2" ry="2" fill="none" stroke="currentColor" stroke-linecap="square" stroke-miterlimit="10" stroke-width="2"></rect></g></svg>"#
}

fn settings_icon() -> &'static str {
    r#"<svg class="size-[1.2em]" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g fill="currentColor" stroke-linejoin="miter" stroke-linecap="butt"><circle cx="12" cy="12" r="3" fill="none" stroke="currentColor" stroke-linecap="square" stroke-miterlimit="10" stroke-width="2"></circle><path d="m22,13.25v-2.5l-2.318-.966c-.167-.581-.395-1.135-.682-1.654l.954-2.318-1.768-1.768-2.318.954c-.518-.287-1.073-.515-1.654-.682l-.966-2.318h-2.5l-.966,2.318c-.581.167-1.135.395-1.654.682l-2.318-.954-1.768,1.768.954,2.318c-.287.518-.515,1.073-.682,1.654l-2.318.966v2.5l2.318.966c.167.581.395,1.135.682,1.654l-.954,2.318,1.768,1.768,2.318-.954c.518.287,1.073.515,1.654.682l.966,2.318h2.5l.966-2.318c.581-.167,1.135-.395,1.654-.682l2.318.954,1.768-1.768-.954-2.318c.287-.518.515-1.073.682-1.654l2.318-.966Z" fill="none" stroke="currentColor" stroke-linecap="square" stroke-miterlimit="10" stroke-width="2"></path></g></svg>"#
}

fn dock_preview(children: String) -> String {
    rsx! {
        <div class="w-full max-w-sm mx-auto border border-base-300 rounded-box bg-base-100 overflow-hidden relative min-h-[8rem] flex flex-col justify-end" style="transform: scale(1)">
            {children}
        </div>
    }
}

pub async fn page() -> Html<String> {
    let ios_note = r#"<code class="bg-base-200 px-1 rounded">&lt;meta name="viewport" content="viewport-fit=cover"&gt;</code> is required for responsiveness of the dock in iOS."#;

    let sizes_example = rsx! {
        <div class="flex flex-col gap-6">
            <div>
                <p class="text-sm text-gray-600 mb-2">"Extra Small"</p>
                {dock_preview(rsx! {
                    <Dock size=Size::Xs>
                        <DockItem>{home_icon()}</DockItem>
                        <DockItem active=true>{inbox_icon()}</DockItem>
                        <DockItem>{settings_icon()}</DockItem>
                    </Dock>
                })}
            </div>
            <div>
                <p class="text-sm text-gray-600 mb-2">"Small"</p>
                {dock_preview(rsx! {
                    <Dock size=Size::Sm>
                        <DockItem>{home_icon()}</DockItem>
                        <DockItem active=true>{inbox_icon()}</DockItem>
                        <DockItem>{settings_icon()}</DockItem>
                    </Dock>
                })}
            </div>
            <div>
                <p class="text-sm text-gray-600 mb-2">"Medium"</p>
                {dock_preview(rsx! {
                    <Dock size=Size::Md>
                        <DockItem label="Home">{home_icon()}</DockItem>
                        <DockItem active=true label="Inbox">{inbox_icon()}</DockItem>
                        <DockItem label="Settings">{settings_icon()}</DockItem>
                    </Dock>
                })}
            </div>
            <div>
                <p class="text-sm text-gray-600 mb-2">"Large"</p>
                {dock_preview(rsx! {
                    <Dock size=Size::Lg>
                        <DockItem label="Home">{home_icon()}</DockItem>
                        <DockItem active=true label="Inbox">{inbox_icon()}</DockItem>
                        <DockItem label="Settings">{settings_icon()}</DockItem>
                    </Dock>
                })}
            </div>
            <div>
                <p class="text-sm text-gray-600 mb-2">"Extra Large"</p>
                {dock_preview(rsx! {
                    <Dock size=Size::Xl>
                        <DockItem label="Home">{home_icon()}</DockItem>
                        <DockItem active=true label="Inbox">{inbox_icon()}</DockItem>
                        <DockItem label="Settings">{settings_icon()}</DockItem>
                    </Dock>
                })}
            </div>
        </div>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Dock"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Dock (also known as Bottom navigation or Bottom bar) is a UI element that provides navigation options to the user. Dock sticks to the bottom of the screen."
            </p>

            <div class="alert alert-info mb-8">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="h-6 w-6 shrink-0 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                <span>{ios_note}</span>
            </div>


            <div class="divider">"Dock in Various Sizes"</div>
            {sizes_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Dock, DockItem};",
                "use rsx_macros::rsx;",
                "",
                "let html = rsx! {",
                "    <Dock>",
                "        <DockItem label=\"Home\">",
                "            <svg class=\"size-[1.2em]\" ...>\"...\"</svg>",
                "        </DockItem>",
                "        <DockItem active={true} label=\"Inbox\">",
                "            <svg class=\"size-[1.2em]\" ...>\"...\"</svg>",
                "        </DockItem>",
                "        <DockItem label=\"Settings\">",
                "            <svg class=\"size-[1.2em]\" ...>\"...\"</svg>",
                "        </DockItem>",
                "    </Dock>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/dock").await)
}
