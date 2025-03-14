use leptos::view;

use crate::components::typography::heading::HeadingText;
use crate::layouts::app_layout::AppLayout;

#[leptos::component]
pub fn AnalyticsScreen() -> impl leptos::IntoView {
    let header = view! { <HeadingText>Analytics</HeadingText> };

    view! {
        <AppLayout header class="h-[90vh] overflow scroll " active_route="analytics">

            ff
        </AppLayout>
    }
}
