use leptos::prelude::*;
use crate::frontend::global_state::GlobalState;

#[component]
pub fn AddRoleModal(set_show_modal: WriteSignal<bool>) -> impl IntoView {
    let global_state = use_context::<GlobalState>().expect("GlobalState should be provided");
    let (selected_emoji, set_selected_emoji) = signal(String::new());
    let (selected_role, set_selected_role) = signal(String::new());

    let close_modal = move |_| set_show_modal.set(false);

    view! {
        <div class="fixed inset-0 bg-gray-900 bg-opacity-75 flex items-center justify-center z-50">
            <div class="bg-gray-800 rounded-lg shadow-lg p-6 w-96 text-white">
                <h2 class="text-xl font-semibold mb-2">"Option"</h2>
                <p class="mb-4">"Configure your option"</p>
                <div class="mb-4">
                    <label for="emoji" class="block text-sm font-medium text-gray-400">"EMOJI:"</label>
                    <input
                        id="emoji"
                        type="text"
                        readonly=true
                        class="mt-1 block w-full rounded-md bg-gray-700 border-gray-600 text-gray-300 p-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
                        value=move || selected_emoji.get()
                        on:click=move |_| {
                            set_selected_emoji.set("😆".to_string());
                        }
                    />
                </div>
                <div class="mb-4">
                    <label for="role_to_add" class="block text-sm font-medium text-gray-400">"ROLE TO ADD:"</label>
                    <select
                        id="role_to_add"
                        class="mt-1 block w-full rounded-md bg-gray-700 border-gray-600 text-gray-300 p-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
                        on:change=move |ev| set_selected_role.set(event_target_value(&ev))
                    >
                        <option value="">"Select Role"</option>
                        {move || {
                            global_state.admin_roles.get()
                                .into_iter()
                                .map(|role| {
                                    view! {
                                        <option value=role class="bg-gray-700 text-gray-300">{role.clone()}</option>
                                    }
                                })
                                .collect::<Vec<_>>()
                        }}
                    </select>
                </div>
                <div class="mt-4 flex justify-between">
                    <button
                        class="px-4 py-2 text-white bg-indigo-500 rounded hover:bg-indigo-600 focus:outline-none focus:ring-2 focus:ring-blue-300"
                        on:click=move |_| {
                            let new_reaction_role = (selected_emoji.get(), String::from("EmojiName"), selected_role.get());
                            global_state.reaction_roles.update(|roles| roles.push(new_reaction_role));
                            set_show_modal.set(false);
                        }
                    >
                        "Guardar"
                    </button>
                    <button
                        class="px-4 py-2 text-white bg-gray-700 rounded hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-blue-300"
                        on:click=close_modal
                    >
                        "Cerrar"
                    </button>
                </div>
            </div>
        </div>
    }
}