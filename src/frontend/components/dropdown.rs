use leptos::prelude::*;

#[component]
pub fn Dropdown(
    options: Vec<String>,
    index: usize,
    active_dropdown: RwSignal<Option<usize>>,
    allow_multiple: bool, // Nueva propiedad para controlar selección única o múltiple
) -> impl IntoView {
    let (selected_options, set_selected_options) = signal(vec![]);

    let show_options = move || active_dropdown.get() == Some(index);

    view! {
        <div class="relative">
            <div
                class="w-full p-3 border border-gray-600 rounded-lg flex justify-between items-center cursor-pointer hover:border-indigo-500 bg-gray-700"
                on:click=move |_| {
                    active_dropdown.update(|active| {
                        if *active == Some(index) {
                            *active = None; // Cierra si ya está abierto
                        } else {
                            *active = Some(index);
                        }
                    });
                }
            >
                <span class="text-gray-200">
                    {move || if selected_options.get().is_empty() {
                        "Select options...".to_string()
                    } else {
                        selected_options.get().join(", ")
                    }}
                </span>
                <i class={move || if show_options() { "icon-chevron-up" } else { "icon-chevron-down" }}></i>
            </div>
            <div
                class={move || format!(
                    "absolute mt-2 w-full bg-gray-700 border border-gray-600 rounded-lg shadow-lg transition-all duration-200 z-50 overflow-hidden {}",
                    if show_options() { "max-h-60 opacity-100 overflow-y-auto" } else { "max-h-0 opacity-0" }
                )}
            >
                {options.clone().into_iter().map(|option| {
                    let option_clone = option.clone();
                    let is_selected = move || selected_options.get().contains(&option_clone);

                    view! {
                        <div
                            class="flex justify-between items-center p-3 hover:bg-gray-600 cursor-pointer"
                            on:click=move |_| {
                                set_selected_options.update(|current| {
                                    if allow_multiple {
                                        if current.contains(&option) {
                                            current.retain(|o| *o != option);
                                        } else {
                                            current.push(option.clone());
                                        }
                                    } else {
                                        if current.contains(&option) {
                                            current.clear();
                                        } else {
                                            current.clear();
                                            current.push(option.clone());
                                        }
                                    }
                                });
                            }
                        >
                            <span class="text-gray-200">{option.clone()}</span>
                            <i class={move || if is_selected() { "icon-check text-indigo-500" } else { "" }}></i>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}