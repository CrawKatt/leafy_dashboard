use leptos::prelude::*;

#[derive(Clone, Debug)]
pub struct GlobalState {
    pub admin_roles: RwSignal<Vec<String>>,
    pub forbidden_user: RwSignal<String>,
    pub forbidden_role: RwSignal<String>,
    pub timeout_time: RwSignal<String>,
    pub welcome_channel: RwSignal<String>,
    pub ooc_channel: RwSignal<String>,
    pub logs_channel: RwSignal<String>,
    pub exceptions_channel: RwSignal<String>,
    pub welcome_message: RwSignal<String>,
    pub timeout_message: RwSignal<String>,
    pub warn_message: RwSignal<String>,
    pub reaction_roles: RwSignal<Vec<(String, String, String)>>
}

impl GlobalState {
    pub fn new() -> Self {
        Self {
            admin_roles: RwSignal::new(vec![String::new()]),
            forbidden_user: RwSignal::new(String::new()),
            forbidden_role: RwSignal::new(String::new()),
            timeout_time: RwSignal::new(String::new()),
            welcome_channel: RwSignal::new(String::new()),
            ooc_channel: RwSignal::new(String::new()),
            logs_channel: RwSignal::new(String::new()),
            exceptions_channel: RwSignal::new(String::new()),
            welcome_message: RwSignal::new(String::new()),
            timeout_message: RwSignal::new(String::new()),
            warn_message: RwSignal::new(String::new()),
            reaction_roles: RwSignal::new(Vec::new()),
        }
    }
}

pub fn provide_global_state() {
    provide_context(GlobalState::new())
}