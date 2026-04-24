use axum::response::Html;
use rsx_macros::rsx;
use rsxui::components::{
    Chat, ChatBubble, ChatFooter, ChatHeader, ChatImage, ChatPlacement, Color,
};

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <Chat placement=ChatPlacement::Start>
            <ChatBubble>"Hello! How are you?"</ChatBubble>
        </Chat>
    };

    let end_example = rsx! {
        <Chat placement=ChatPlacement::End>
            <ChatBubble>"I'm doing great, thanks!"</ChatBubble>
        </Chat>
    };

    let full_example = rsx! {
        <div class="flex flex-col gap-4">
            <Chat placement=ChatPlacement::Start>
                <ChatImage>
                    <div class="w-10 rounded-full">
                        <img src="https://picsum.photos/200" alt="User" />
                    </div>
                </ChatImage>
                <ChatHeader>"Obi-Wan Kenobi"</ChatHeader>
                <ChatBubble color=Color::Primary>"You were the chosen one!"</ChatBubble>
                <ChatFooter>"2 hours ago"</ChatFooter>
            </Chat>
            <Chat placement=ChatPlacement::End>
                <ChatImage>
                    <div class="w-10 rounded-full">
                        <img src="https://picsum.photos/200" alt="User" />
                    </div>
                </ChatImage>
                <ChatHeader>"Anakin Skywalker"</ChatHeader>
                <ChatBubble color=Color::Secondary>"I hate you!"</ChatBubble>
                <ChatFooter>"2 hours ago"</ChatFooter>
            </Chat>
        </div>
    };

    let colors_example = rsx! {
        <div class="flex flex-col gap-2">
            <Chat>
                <ChatBubble color=Color::Info>"Info message"</ChatBubble>
            </Chat>
            <Chat>
                <ChatBubble color=Color::Success>"Success message"</ChatBubble>
            </Chat>
            <Chat>
                <ChatBubble color=Color::Warning>"Warning message"</ChatBubble>
            </Chat>
            <Chat>
                <ChatBubble color=Color::Error>"Error message"</ChatBubble>
            </Chat>
        </div>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Chat Bubble"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Chat bubbles are used to show one line of conversation and all its data."
            </p>

            <div class="divider">"Basic Chat"</div>
            <p class="text-sm text-gray-600 mb-4">"Simple chat message on the left."</p>
            {basic_example}

            <div class="divider">"Chat End"</div>
            <p class="text-sm text-gray-600 mb-4">"Chat message on the right."</p>
            {end_example}

            <div class="divider">"Full Chat Example"</div>
            <p class="text-sm text-gray-600 mb-4">"With avatar, header, and footer."</p>
            {full_example}

            <div class="divider">"Chat Bubble Colors"</div>
            <p class="text-sm text-gray-600 mb-4">"Different colors for message bubbles."</p>
            {colors_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Chat, ChatBubble, ChatPlacement};",
                "use rsx_macros::rsx;",
                "",
                "let html = rsx! {",
                "    <Chat placement={ChatPlacement::Start}>",
                "        <ChatBubble>\"Hello!\"</ChatBubble>",
                "    </Chat>",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/chat").await)
}
