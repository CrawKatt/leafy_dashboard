use leptos::prelude::*;

#[component]
pub fn Card(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class="p-6 bg-gray-800 rounded-lg shadow-md">
            <h2 class="text-lg font-bold text-indigo-400">{title}</h2>
            <div class="mt-4">{children()}</div>
        </div>
    }
}