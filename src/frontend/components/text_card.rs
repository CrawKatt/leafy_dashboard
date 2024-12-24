use leptos::prelude::*;

#[component]
pub fn TextCard(
    title: &'static str,
    placeholder: &'static str,
    on_change: WriteSignal<String>
) -> impl IntoView {
    view! {
        <div class="p-4 border border-gray-600 rounded-lg bg-gray-700 shadow-md">
            <h2 class="text-lg font-bold text-gray-200 mb-2">{title}</h2>
            <textarea
                class="w-full p-3 rounded-lg bg-gray-800 border border-gray-600 text-gray-200 resize-none focus:outline-none focus:ring focus:ring-indigo-500"
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