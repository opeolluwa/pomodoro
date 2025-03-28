// use leptos::task::spawn_local;
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use thaw::ConfigProvider;

use crate::screens::{
    analytics::AnalyticsScreen,
    authentication::{sign_in::SignInScreen, sign_up::SignUpScreen, welcome::WelcomeScreen},
    focus::FocusScreen,
    home::HomeScreen,
    notification::NotificationScreen,
    profile::ProfileScreen,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <ConfigProvider>
            <Router>
                <Routes transition=true fallback=|| "not found ">
                    <Route path=path!("/") view=WelcomeScreen />
                    <Route path=path!("/sign-in") view=SignInScreen />
                    <Route path=path!("/sign-up") view=SignUpScreen />

                    <Route path=path!("/home") view=HomeScreen />
                    <Route path=path!("/notification") view=NotificationScreen />
                    <Route path=path!("/focus") view=FocusScreen />
                    <Route path=path!("/profile") view=ProfileScreen />
                    <Route path=path!("/analytics") view=AnalyticsScreen />

                </Routes>
            </Router>
        </ConfigProvider>
    }
}
