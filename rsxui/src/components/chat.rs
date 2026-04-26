// Chat Bubble Component
// Based on DaisyUI Chat: https://daisyui.com/components/chat/

use crate::components::Color;
use enum_stringify::EnumStringify;
use rsx::{classes, component, rsx};

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

#[component]
pub fn Chat(
    #[builder(default)] placement: ChatPlacement,
    #[builder(default)] class: String,
    children: String,
) -> String {
    rsx! {
        <div class={classes!("chat", placement, class)}>
            {children}
        </div>
    }
}

// ============================================
// ChatImage - Avatar image
// ============================================

#[component]
pub fn ChatImage(children: String) -> String {
    rsx! {
        <div class="chat-image avatar">
            {children}
        </div>
    }
}

// ============================================
// ChatHeader - Message header
// ============================================

#[component]
pub fn ChatHeader(children: String) -> String {
    rsx! {
        <div class="chat-header">{children}</div>
    }
}

// ============================================
// ChatBubble - Message bubble
// ============================================

#[component]
pub fn ChatBubble(
    #[builder(default)] color: Color,
    #[builder(default)] class: String,
    children: String,
) -> String {
    rsx! {
        <div class={classes!("chat-bubble", color.prefix("chat-bubble"), class)}>
            {children}
        </div>
    }
}

// ============================================
// ChatFooter - Message footer
// ============================================

#[component]
pub fn ChatFooter(children: String) -> String {
    rsx! {
        <div class="chat-footer">{children}</div>
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
