use crate::frontend::pages::home::Home;
use crate::frontend::pages::dashboard::Dashboard;
use crate::frontend::pages::server_settings::ServerSettings;

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{FlatRoutes, Route, Router},
    path
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/tailwind_actix.css"/>
        <Link rel="shortcut icon" type_="image/png" href="/favicon.png"/>
        <Router>
            <FlatRoutes fallback=|| "Page not found.">
                <Route path=path!("") view=Home/>
                <Route path=path!("/dashboard") view=Dashboard/>
                <Route path=path!("/dashboard/:guild_id") view=ServerSettings/>
            </FlatRoutes>
        </Router>
    }
}