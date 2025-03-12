use crate::components::typography::heading::HeadingText;
use crate::layouts::app_layout::AppLayout;
use leptos::view;

#[leptos::component]
pub fn AnalyticsScreen() -> impl leptos::IntoView {
    let header = view! { <HeadingText>Analytics</HeadingText> };

    view! {
        <AppLayout header class="h-[90vh]" active_route="analytics">

            ff
        </AppLayout>
    }
}
