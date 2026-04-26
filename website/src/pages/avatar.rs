use axum::response::Html;
use rsx::rsx;
use rsxui::components::{Avatar, AvatarGroup, AvatarStatus};

use crate::layout;

pub async fn page() -> Html<String> {
    let basic_example = rsx! {
        <Avatar src="https://picsum.photos/200" alt="User" inner_class="w-24 rounded-xl" />
    };

    let sizes_example = rsx! {
        <div class="flex gap-4 items-center">
            <Avatar src="https://picsum.photos/200" alt="Avatar" inner_class="w-32 rounded" />
            <Avatar src="https://picsum.photos/200" alt="Avatar" inner_class="w-20 rounded" />
            <Avatar src="https://picsum.photos/200" alt="Avatar" inner_class="w-16 rounded" />
            <Avatar src="https://picsum.photos/200" alt="Avatar" inner_class="w-8 rounded" />
        </div>
    };

    let status_example = rsx! {
        <div class="flex gap-4">
            <Avatar src="https://picsum.photos/200" alt="Online" status=AvatarStatus::Online inner_class="w-20 rounded-full" />
            <Avatar src="https://picsum.photos/200" alt="Offline" status=AvatarStatus::Offline inner_class="w-20 rounded-full" />
            <Avatar status=AvatarStatus::Placeholder inner_class="bg-neutral text-neutral-content w-20 rounded-full" placeholder="AB" />
        </div>
    };

    let group_example = rsx! {
        <AvatarGroup class="-space-x-6">
            <Avatar src="https://picsum.photos/200" alt="User 1" inner_class="w-12" />
            <Avatar src="https://picsum.photos/200" alt="User 2" inner_class="w-12" />
            <Avatar src="https://picsum.photos/200" alt="User 3" inner_class="w-12" />
            <Avatar src="https://picsum.photos/200" alt="User 4" inner_class="w-12" />
            <Avatar status=AvatarStatus::Placeholder inner_class="bg-neutral text-neutral-content w-12" placeholder="+99" />
        </AvatarGroup>
    };

    let content = rsx! {
        <div class="max-w-4xl">
            <div class="flex items-center justify-between mb-8">
                <h1 class="text-4xl font-bold">"Avatar"</h1>
                <span class="badge badge-lg badge-primary">"DaisyUI"</span>
            </div>

            <p class="text-lg mb-8">
                "Avatars are used to show a thumbnail representation of a user."
            </p>

            <div class="divider">"Basic Avatar"</div>
            {basic_example}

            <div class="divider">"Avatar Sizes"</div>
            <p class="text-sm text-gray-600 mb-4">"Different sizes using Tailwind classes."</p>
            {sizes_example}

            <div class="divider">"Avatar Status"</div>
            <p class="text-sm text-gray-600 mb-4">"Online, offline, and placeholder indicators."</p>
            {status_example}

            <div class="divider">"Avatar Group"</div>
            <p class="text-sm text-gray-600 mb-4">"Overlapping avatars in a group."</p>
            {group_example}

            <div class="divider">"Code Example"</div>
            {crate::html_utils::code_example(&[
                "use rsxui::components::{Avatar, AvatarGroup, AvatarStatus};",
                "use rsx::rsx;",
                "",
                "let html = rsx! {",
                "    <Avatar src=\"https://example.com/avatar.jpg\" alt=\"User\" />",
                "};",
                "",
                "// With status",
                "let html = rsx! {",
                "    <Avatar",
                "        src=\"https://example.com/avatar.jpg\"",
                "        alt=\"User\"",
                "        status={AvatarStatus::Online}",
                "    />",
                "};",
            ])}
        </div>
    };

    Html(layout::drawer(content, "/components/avatar").await)
}
