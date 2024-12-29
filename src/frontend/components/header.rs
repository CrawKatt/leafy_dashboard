use leptos::prelude::*;
use crate::frontend::components::save_changes_button::SaveChangesButton;

#[component]
pub fn Header(title: &'static str) -> impl IntoView {
    view! {
        <div class="flex justify-between items-center p-6 bg-gray-800">
            <h1 class="text-xl font-bold">{title}</h1>
            <SaveChangesButton/>
        </div>
    }
}