// use leptos::task::spawn_local;
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use thaw::ConfigProvider;

use crate::screens::{
    analytics::AnalyticsScreen, focus::FocusScreen, home::HomeScreen,
    notification::NotificationScreen, profile::ProfileScreen,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <ConfigProvider>
        <Router>
            <Routes transition=true fallback=|| "not found ">
                <Route path=path!("/") view=HomeScreen />
                <Route path=path!("/notification") view=NotificationScreen />
                <Route path=path!("/focus") view=FocusScreen />
                <Route path=path!("/profile") view=ProfileScreen />
                <Route path=path!("/analytics") view=AnalyticsScreen />

            </Routes>
        </Router>
        </ConfigProvider>
    }
}
