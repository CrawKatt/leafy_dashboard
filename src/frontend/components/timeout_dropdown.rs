use crate::frontend::components::card::Card;
use crate::frontend::components::dropdown::Dropdown;
use leptos::prelude::*;

#[component]
pub fn TimeoutDropdown(
    title: &'static str,
    index: usize,
    allow_multiple: bool,
    duration: Vec<String>,
    active_dropdown: RwSignal<Option<usize>>,
    on_change: Callback<Vec<String>>,
) -> impl IntoView {
    view! {
        <Card title=title>
            {move || {
                let duration_mapping: Vec<(String, String)> = duration
                    .iter()
                    .map(|dur| {
                        match dur.as_str() {
                            "1 Minuto" => (dur.clone(), "60".to_string()),
                            "5 Minutos" => (dur.clone(), "300".to_string()),
                            "10 Minutos" => (dur.clone(), "600".to_string()),
                            "1 Hora" => (dur.clone(), "3600".to_string()),
                            "1 Día" => (dur.clone(), "86400".to_string()),
                            "1 Semana" => (dur.clone(), "604800".to_string()),
                            _ => (dur.clone(), "0".to_string()),
                        }
                    })
                    .collect();
                let options = duration_mapping
                    .iter()
                    .map(|(display, _)| display.clone())
                    .collect::<Vec<String>>();
                view! {
                    <Dropdown
                        options=options
                        index=index
                        active_dropdown=active_dropdown
                        allow_multiple=allow_multiple
                        on_change=Callback::new(move |selected_display: Vec<String>| {
                            let selected_seconds: Vec<String> = duration_mapping
                                .iter()
                                .filter(|(display, _)| selected_display.contains(display))
                                .map(|(_, seconds)| seconds.clone())
                                .collect();
                            on_change.run(selected_seconds);
                        })
                    />
                }
            }}
        </Card>
    }
}