use leptos::prelude::*;

#[component]
pub fn Header(title: &'static str) -> impl IntoView {
    view! {
        <div class="flex justify-between items-center p-6 bg-gray-800">
            <h1 class="text-xl font-bold">{title}</h1>
            <button class="py-2 px-4 text-white bg-indigo-500 rounded hover:bg-indigo-600">
                "Save Changes"
            </button>
        </div>
    }
}