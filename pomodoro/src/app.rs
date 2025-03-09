// use leptos::task::spawn_local;
use leptos::{html::Style, prelude::*};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::screens::home::HomeScreen;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes transition=true fallback=|| "not found ">
                <Route path=path!("/") view=HomeScreen />
            </Routes>
        </Router>
    }
}
