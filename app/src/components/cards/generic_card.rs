use leptos::prelude::Children;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::view;

#[leptos::component]
pub fn GenericCard(
    children: Children,
    #[prop(optional)] class: &'static str,
) -> impl leptos::IntoView {
    view! { <div class=format!("bg-white rounded-[10px] p-[20px] {}", class)>{children()}</div> }
}
