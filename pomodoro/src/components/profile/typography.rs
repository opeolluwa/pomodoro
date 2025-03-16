use leptos::prelude::{Children, ClassAttribute, ElementChild};
use leptos::view;

#[leptos::component]
pub fn SectionTitle(children: Children) -> impl leptos::IntoView {
    view! {
        <h4 class="font-semibold leading-[18px] text-[#525772] my-[2px]">{children()} </h4>
    }
}
