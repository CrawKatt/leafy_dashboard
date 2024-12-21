use leptos::prelude::*;

#[component]
pub fn Dropdown(options: Vec<&'static str>) -> impl IntoView {
    let (show_options, set_show_options) = signal(false);
    let (selected_options, set_selected_options) = signal(vec![]);

    view! {
        <div class="relative">
            <div
                class="w-full p-3 border border-gray-600 rounded-lg flex justify-between items-center cursor-pointer hover:border-indigo-500 bg-gray-700"
                on:click=move |_| set_show_options.update(|v| *v = !*v)
            >
                <span class="text-gray-200">
                    {move || if selected_options.get().is_empty() {
                        "Select options...".to_string()
                    } else {
                        selected_options.get().join(", ")
                    }}
                </span>
                <i class={move || if show_options.get() { "icon-chevron-up" } else { "icon-chevron-down" }}></i>
            </div>
            <div
                class={move || format!(
                    "absolute mt-2 w-full bg-gray-700 border border-gray-600 rounded-lg shadow-lg transition-all duration-200 {}",
                    if show_options.get() { "max-h-60 opacity-100" } else { "max-h-0 opacity-0 pointer-events-none" }
                )}
            >
                {options.iter().map(|option| {
                    let option = *option;
                    let is_selected = move || selected_options.get().contains(&option.to_string());

                    view! {
                        <div
                            class="flex justify-between items-center p-3 hover:bg-gray-600 cursor-pointer"
                            on:click=move |_| {
                                set_selected_options.update(|current| {
                                    if current.contains(&option.to_string()) {
                                        current.retain(|o| o != &option);
                                    } else {
                                        current.push(option.to_string());
                                    }
                                });
                            }
                        >
                            <span class="text-gray-200">{option}</span>
                            <i class={move || if is_selected() { "icon-check" } else { "" }}></i>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

