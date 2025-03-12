use crate::components::typography::heading::HeadingText;
use crate::layouts::app_layout::AppLayout;
use leptos::view;

#[leptos::component]
pub fn ProfileScreen() -> impl leptos::IntoView {
    let header = view! { <HeadingText>Profile</HeadingText> };

    view! {
        <AppLayout header class="h-[90vh]" active_route="profile">

            ff
        </AppLayout>
    }
}
