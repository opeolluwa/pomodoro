use leptos::prelude::StyleAttribute;
use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::IntoView;
use leptos::{prelude::Children, view};

#[leptos::component]
pub fn AppLayout<F>(
    children: Children,
    header: F,
    #[prop(optional)] class: &'static str,
) -> impl leptos::IntoView
where
    F: IntoView,
{
    view! {
        <header class="px-[24px] py-[17px] bg-white">{header}</header>
        <main
            class=format!("bg-[#F5F5F5] py-[17px] overflow-y-scroll px-[24px] {}", class)
            style="position:relative"
        >
            {children()}
        </main>
        <nav class="px-[24px]"></nav>
    }
}
