// Avatar Component
// Based on DaisyUI Avatar: https://daisyui.com/components/avatar/

use enum_stringify::EnumStringify;
use rsx::attrs::RenderAttrs;
use rsx::{classes, rsx, ui};

// ============================================
// AvatarStatus - Avatar status modifier
// ============================================

#[derive(Debug, Clone, PartialEq, Default, EnumStringify)]
#[enum_stringify(case = "kebab", prefix = "avatar-")]
pub enum AvatarStatus {
    #[default]
    #[enum_stringify(rename = "")]
    Default,
    Online,
    Offline,
    Placeholder,
}

// ============================================
// Avatar - User thumbnail image
// ============================================

#[ui]
pub fn Avatar(
    #[builder(into)] src: Option<String>,
    #[builder(default)] alt: String,
    #[builder(default)] status: AvatarStatus,
    #[builder(default)] class: String,
    #[builder(default)] inner_class: String,
    #[builder(into)] placeholder: Option<String>,
) -> String {
    rsx! {
        <div class={classes!("avatar", props.status, props.class)} {props.render_attrs()}>
            <div class={props.inner_class.clone()}>
                {
                    if let Some(ref img_src) = props.src {
                       rsx!{ <img src={img_src.clone()} alt={props.alt.clone()} /> }
                    }
                }
                {
                    if let Some(ref placeholder) = props.placeholder {
                       rsx!{ <span>{placeholder}</span> }
                    }
                }
            </div>
        </div>
    }
}

// ============================================
// AvatarGroup - Group of overlapping avatars
// ============================================

#[ui]
pub fn AvatarGroup(#[builder(default)] class: String, children: String) -> String {
    rsx! {
        <div class={classes!("avatar-group", props.class)} {props.render_attrs()}>
            {props.children}
        </div>
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
    async fn test_avatar_basic() {
        let html = rsx! {
            <Avatar src="https://picsum.photos/200" alt="User" />
        };
        assert!(html.contains("class=\"avatar\""));
        assert!(html.contains("src=\"https://picsum.photos/200\""));
        assert!(html.contains("alt=\"User\""));
    }

    #[tokio::test]
    async fn test_avatar_online() {
        let html = rsx! {
            <Avatar src="https://picsum.photos/200" alt="User" status=AvatarStatus::Online />
        };
        assert!(html.contains("avatar-online"));
    }

    #[tokio::test]
    async fn test_avatar_offline() {
        let html = rsx! {
            <Avatar src="https://picsum.photos/200" alt="User" status=AvatarStatus::Offline />
        };
        assert!(html.contains("avatar-offline"));
    }

    #[tokio::test]
    async fn test_avatar_placeholder() {
        let html = rsx! {
            <Avatar src="" alt="User" status=AvatarStatus::Placeholder />
        };
        assert!(html.contains("avatar-placeholder"));
    }

    #[tokio::test]
    async fn test_avatar_group() {
        let html = rsx! {
            <AvatarGroup>
                <Avatar src="https://picsum.photos/200" alt="User 1" />
                <Avatar src="https://picsum.photos/200" alt="User 2" />
            </AvatarGroup>
        };
        assert!(html.contains("avatar-group"));
    }
}
