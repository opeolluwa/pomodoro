use leptos::IntoView;
use leptos::{prelude::*, view};

#[leptos::component]
pub fn AuthenticationLayout(
    children: Children,
    #[prop(optional)] class: &'static str,
) -> impl leptos::IntoView {
    view! {
        <main
            class=format!("bg-white py-[24px]  overflow-y-scroll px-[24px] {}", class)
            style="position:relative"
        >

            {children()}
        </main>
    }
}
