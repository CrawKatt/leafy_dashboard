use leptos::prelude::*;

#[component]
pub fn TextCard(
    title: &'static str,
    placeholder: &'static str,
    on_change: RwSignal<String>
) -> impl IntoView {
    view! {
        <div class="p-4 bg-gray-700 rounded-lg border border-gray-600 shadow-md">
            <h2 class="mb-2 text-lg font-bold text-gray-200">{title}</h2>
            <textarea
                class="p-3 w-full text-gray-200 bg-gray-800 rounded-lg border border-gray-600 resize-none focus:ring focus:ring-indigo-500 focus:outline-none"
                rows="4"
                placeholder=placeholder
                on:input=move |e| {
                    let value = event_target_value(&e);
                    on_change.set(value)
                }
            ></textarea>
        </div>
    }
}