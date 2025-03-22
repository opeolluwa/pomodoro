use leptos::prelude::view;
use leptos::prelude::Children;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;

#[leptos::component]
pub fn HeadingText(
    children: Children,
    #[prop(optional)] class: &'static str,
) -> impl leptos::IntoView {
    view! { <h2 class=format!("font-[18px] leading-[22px] font-semibold {}", class)>{children()}</h2> }
}
