use leptos::prelude::*;
#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <div class="w-64 bg-gray-800 p-4">
            <h2 class="text-lg font-bold text-indigo-400">Bot Config</h2>
            <ul class="mt-6 space-y-4">
                <li class="flex items-center gap-2 cursor-pointer hover:text-indigo-400">
                    <i class="icon-settings" /> "General Settings"
                </li>
                <li class="flex items-center gap-2 cursor-pointer hover:text-indigo-400">
                    <i class="icon-shield" /> "Roles & Permissions"
                </li>
                <li class="flex items-center gap-2 cursor-pointer hover:text-indigo-400">
                    <i class="icon-user" /> "User Management"
                </li>
                <li class="flex items-center gap-2 cursor-pointer hover:text-indigo-400">
                    <i class="icon-hash" /> "Channels"
                </li>
                <li class="flex items-center gap-2 cursor-pointer hover:text-indigo-400">
                    <i class="icon-message" /> "Messages"
                </li>
            </ul>
        </div>
    }
}