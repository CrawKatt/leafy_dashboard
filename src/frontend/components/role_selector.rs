use leptos::prelude::*;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub color: String,
}

#[component]
pub fn RoleSelector() -> impl IntoView {
    let roles = vec![
        ("Admin", "red"),
        ("Moderator", "green"),
        ("Helper", "blue"),
        ("Member", "gray"),
        ("Muted", "gray"),
    ];

    let (show_options, set_show_options) = signal(false);
    let (selected_roles, set_selected_roles) = signal(vec![]);

    let toggle_options = move |_| {
        set_show_options.update(|value| *value = !*value);
    };

    view! {
        <div class="p-6 bg-gray-800 rounded-xl shadow-md">
            <div
                class="w-full p-3 border border-gray-600 rounded-lg flex justify-between items-center cursor-pointer hover:border-indigo-500 bg-gray-700"
                on:click=toggle_options
            >
                <span class="text-gray-200">
                    {move || if selected_roles.get().is_empty() {
                        "Select roles...".to_string()
                    } else {
                        selected_roles.get().join(", ")
                    }}
                </span>
                <span class="text-gray-400">
                    {move || format!("{}/5", selected_roles.get().len())}
                </span>
            </div>
            <div class=move || {
                if show_options.get() {
                    "absolute z-10 w-full mt-2 bg-gray-700 border border-gray-600 rounded-lg shadow-lg"
                } else {
                    "hidden"
                }
            }>
                <div class="p-2 border-b border-gray-600">
                    <div class="relative">
                        <input
                            type="text"
                            placeholder="Search roles..."
                            class="w-full pl-9 pr-4 py-2 bg-gray-800 border border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500 text-gray-200 placeholder-gray-400"
                        />
                    </div>
                </div>
                <div class="max-h-60 overflow-y-auto">
                    {roles.into_iter().map(move |(role, color)| {
                        let role_name = role.to_string();
                        let is_selected = selected_roles.get().contains(&role_name);
                        view! {
                            <div
                                class="flex items-center justify-between px-4 py-2 hover:bg-gray-600 cursor-pointer"
                                on:click=move |_| {
                                    if is_selected {
                                        set_selected_roles.update(|roles| roles.retain(|r| r != &role_name));
                                    } else if selected_roles.get().len() < 5 {
                                        set_selected_roles.update(|roles| roles.push(role_name.clone()));
                                    }
                                }
                            >
                                <div class="flex items-center gap-2">
                                    <div
                                        class=format!("w-3 h-3 rounded-full bg-{}-500", color)
                                    ></div>
                                    <span class="text-gray-200">{role}</span>
                                </div>
                                <svg
                                    class=move || if is_selected { "w-4 h-4 text-indigo-400" } else { "w-4 h-4 text-gray-400" }
                                    fill="none"
                                    stroke="currentColor"
                                    viewBox="0 0 24 24"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d=move || if is_selected {
                                            "M5 13l4 4L19 7"
                                        } else {
                                            ""
                                        }
                                    />
                                </svg>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
            <p class="text-sm text-gray-400 mt-2">
                "Click to select up to 5 roles"
            </p>
        </div>
    }
}

/*
#[component]
pub fn RoleSelector(
    roles: Vec<Role>,
    initial_selected_roles: Vec<String>,
    max_selections: usize,
    label: String,
    on_change: Box<dyn Fn(Vec<String>) + 'static>,
) -> impl IntoView {
    // Señales reactivas para manejar el estado
    let (is_open, set_is_open) = signal(false);
    let (search_term, set_search_term) = signal(String::new());
    let (selected_roles, set_selected_roles) = signal(initial_selected_roles);

    let filtered_roles = move || {
        roles
            .iter()
            .filter(|role| role.name.to_lowercase().contains(&search_term.get().to_lowercase()))
            .cloned()
            .collect::<Vec<Role>>()
    };

    let toggle_role = move |role_id: String| {
        let mut current = selected_roles.get();
        if current.contains(&role_id) {
            current.retain(|&id| id != role_id);
        } else if current.len() < max_selections {
            current.push(role_id);
        }
        set_selected_roles.set(current.clone());
        on_change(current); // Notificar cambios
    };

    let get_selected_role_names = move || {
        selected_roles
            .get()
            .iter()
            .filter_map(|id| roles.iter().find(|role| role.id == *id).map(|role| role.name))
            .collect::<Vec<_>>()
            .join(", ")
    };

    view! {
        <div class="bg-gray-800 rounded-xl shadow-md p-6">
            <div class="flex items-center gap-2 mb-4">
                <div class="w-5 h-5 text-indigo-400">"🛡️"</div>
                <h2 class="text-lg font-semibold text-white">{label}</h2>
            </div>

            <div class="relative">
                <div
                    class="w-full p-3 border border-gray-600 rounded-lg flex justify-between items-center cursor-pointer hover:border-indigo-500 bg-gray-700"
                    on:click=move |_| set_is_open.set(!is_open.get())
                >
                    <span class="text-gray-200">
                        {move || if !selected_roles.get().is_empty() {
                            get_selected_role_names()
                        } else {
                            "Select roles...".to_string()
                        }}
                    </span>
                    <span class="text-gray-400">
                        {move || format!("{}/{}", selected_roles.get().len(), max_selections)}
                    </span>
                </div>

                {move || {
                    view! {
                        <div class="absolute z-10 w-full mt-2 bg-gray-700 border border-gray-600 rounded-lg shadow-lg">
                            <div class="p-2 border-b border-gray-600">
                                <div class="relative">
                                    <div class="absolute left-3 top-2.5 w-4 h-4 text-gray-400">"🔍"</div>
                                    <input
                                        type="text"
                                        placeholder="Search roles..."
                                        class="w-full pl-9 pr-4 py-2 bg-gray-800 border border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500 text-gray-200 placeholder-gray-400"
                                        prop:value=search_term.get()
                                        on:input=move |e| set_search_term.set(event_target_value(&e))
                                    />
                                </div>
                            </div>

                            <div class="max-h-60 overflow-y-auto">
                                {move || filtered_roles().into_iter().map(|role| {
                                    let is_selected = selected_roles.get().contains(&role.id);
                                    view! {
                                        <div
                                            id={format!("role-id {}",role.id)}
                                            class="flex items-center justify-between px-4 py-2 hover:bg-gray-600 cursor-pointer"
                                            on:click=move |_| toggle_role(role.id)
                                        >
                                            <div class="flex items-center gap-2">
                                                <div
                                                    class="w-3 h-3 rounded-full"
                                                    style={format!("background-color: {}", role.color)}
                                                />
                                                <span class="text-gray-200">{role.name}</span>
                                            </div>
                                            {if is_selected {
                                                view! { <div class="w-4 h-4 text-indigo-400">"✔️"</div> }
                                            } else {
                                                view! { <div class="w-4 h-4 text-indigo-400">"X"</div> }
                                            }}
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        </div>
                    }
                }}
            </div>

            <p class="text-sm text-gray-400 mt-2">
                {format!("Click to select up to {} roles", max_selections)}
            </p>
        </div>
    }
}
*/