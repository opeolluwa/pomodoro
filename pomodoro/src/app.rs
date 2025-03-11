// use leptos::task::spawn_local;
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::screens::{home::HomeScreen, notification::NotificationScreen};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes transition=true fallback=|| "not found ">
                <Route path=path!("/") view=HomeScreen />
                <Route path=path!("/notification") view=NotificationScreen />

            </Routes>
        </Router>
    }
}
