use leptos::prelude::view;
use leptos::prelude::Children;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;

#[leptos::component]
pub fn BaseText(
    children: Children,
    #[prop(optional)] class: &'static str,
) -> impl leptos::IntoView {
    view! { <div class=format!("text-[#B2B6CB] leading-[25px]  {}", class)>{children()}</div> }
}
