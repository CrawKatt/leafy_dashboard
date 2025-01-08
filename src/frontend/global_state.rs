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
}

pub fn use_global_state() -> GlobalState {
    let admin_roles = RwSignal::new(vec![String::new()]);
    let forbidden_user = RwSignal::new(String::new());
    let forbidden_role = RwSignal::new(String::new());
    let timeout_time = RwSignal::new(String::new());
    let welcome_channel = RwSignal::new(String::new());
    let ooc_channel = RwSignal::new(String::new());
    let logs_channel = RwSignal::new(String::new());
    let exceptions_channel = RwSignal::new(String::new());
    let welcome_message = RwSignal::new(String::new());
    let timeout_message = RwSignal::new(String::new());
    let warn_message = RwSignal::new(String::new());

    GlobalState {
        admin_roles,
        forbidden_user,
        forbidden_role,
        timeout_time,
        welcome_channel,
        ooc_channel,
        logs_channel,
        exceptions_channel,
        welcome_message,
        timeout_message,
        warn_message,
    }
}

pub fn provide_global_state() {
    provide_context(use_global_state())
}