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
        <header class="px-[24px] py-[17px] bg-white" style="padding: 0 24px">
            {header}
        </header>
        <main
            class=format!("bg-[#F5F5F5] overflow-y-scroll px-[24px] {}", class)
            style="padding: 0 24px"
        >
            {children()}
        </main>
        <nav class="px-[24px]"></nav>
    }
}
