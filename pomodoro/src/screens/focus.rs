use leptos::view;

use crate::components::typography::heading::HeadingText;
use crate::layouts::app_layout::AppLayout;

#[leptos::component]
pub fn FocusScreen() -> impl leptos::IntoView {
    let header = view! { <HeadingText>Focus</HeadingText> };

    view! {
        <AppLayout header class="h-[90vh] overflow scroll " active_route="focus">

            ff
        </AppLayout>
    }
}
