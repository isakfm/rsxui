// Chat Bubble Component
// Based on DaisyUI Chat: https://daisyui.com/components/chat/

use crate::components::Color;
use enum_stringify::EnumStringify;
use rsx::attrs::RenderAttrs;
use rsx::{classes, rsx, ui};

// ============================================
// ChatPlacement - Message placement
// ============================================

#[derive(Debug, Clone, PartialEq, Default, EnumStringify)]
#[enum_stringify(case = "kebab", prefix = "chat-")]
pub enum ChatPlacement {
    #[default]
    Start,
    End,
}

// ============================================
// Chat - Chat message wrapper
// ============================================

#[ui]
pub fn Chat(
    #[builder(default)] placement: ChatPlacement,
    #[builder(default)] class: String,
    children: String,
) -> String {
    rsx! {
        <div class={classes!("chat", props.placement, props.class)} {props.render_attrs()}>
            {props.children}
        </div>
    }
}

// ============================================
// ChatImage - Avatar image
// ============================================

#[ui]
pub fn ChatImage(children: String) -> String {
    rsx! {
        <div class="chat-image avatar" {props.render_attrs()}>
            {props.children}
        </div>
    }
}

// ============================================
// ChatHeader - Message header
// ============================================

#[ui]
pub fn ChatHeader(children: String) -> String {
    rsx! {
        <div class="chat-header" {props.render_attrs()}>{props.children}</div>
    }
}

// ============================================
// ChatBubble - Message bubble
// ============================================

#[ui]
pub fn ChatBubble(
    #[builder(default)] color: Color,
    #[builder(default)] class: String,
    children: String,
) -> String {
    rsx! {
        <div class={classes!("chat-bubble", props.color.prefix("chat-bubble"), props.class)} {props.render_attrs()}>
            {props.children}
        </div>
    }
}

// ============================================
// ChatFooter - Message footer
// ============================================

#[ui]
pub fn ChatFooter(children: String) -> String {
    rsx! {
        <div class="chat-footer" {props.render_attrs()}>{props.children}</div>
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
    async fn test_chat_start() {
        let html = rsx! {
            <Chat>
                <ChatBubble>"Hello!"</ChatBubble>
            </Chat>
        };
        assert!(html.contains("chat-start"));
        assert!(html.contains("chat-bubble"));
        assert!(html.contains("Hello!"));
    }

    #[tokio::test]
    async fn test_chat_end() {
        let html = rsx! {
            <Chat placement=ChatPlacement::End>
                <ChatBubble>"Hi there!"</ChatBubble>
            </Chat>
        };
        assert!(html.contains("chat-end"));
        assert!(html.contains("Hi there!"));
    }

    #[tokio::test]
    async fn test_chat_with_header_footer() {
        let html = rsx! {
            <Chat placement=ChatPlacement::Start>
                <ChatImage>
                    <div class="w-10 rounded-full">
                        <img src="https://picsum.photos/200" />
                    </div>
                </ChatImage>
                <ChatHeader>"Obi-Wan Kenobi"</ChatHeader>
                <ChatBubble color=Color::Primary>"You were the chosen one!"</ChatBubble>
                <ChatFooter>"Delivered"</ChatFooter>
            </Chat>
        };
        assert!(html.contains("chat-image"));
        assert!(html.contains("chat-header"));
        assert!(html.contains("chat-bubble-primary"));
        assert!(html.contains("chat-footer"));
    }

    #[tokio::test]
    async fn test_chat_bubble_colors() {
        let html = rsx! {
            <Chat placement=ChatPlacement::End>
                <ChatBubble color=Color::Success>"Success"</ChatBubble>
            </Chat>
        };
        assert!(html.contains("chat-bubble-success"));
    }
}
