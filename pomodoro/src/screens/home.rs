use crate::components::home::cards::{ActivityCard, TimerCard};
use crate::components::home::header::HomeScreenHeader;
use crate::layouts::app_layout::AppLayout;
use leptos::prelude::ElementChild;
use leptos::{prelude::ClassAttribute, view};

#[leptos::component]
pub fn HomeScreen() -> impl leptos::IntoView {
    let header = HomeScreenHeader();

    view! {
        <AppLayout header class="h-[90vh] overflow scroll" active_route="home">
            <h2 class="text-app-text leading-[14px] font-[600] mt-3">Quick Focus</h2>
            <TimerCard />
            <ActivityCard />
        </AppLayout>
    }
}
