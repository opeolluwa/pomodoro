use leptos::prelude::view;
use leptos::prelude::Children;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;

#[leptos::component]
pub fn HeadingText(children: Children) -> impl leptos::IntoView {
    view! { <h2 class="font-[18px] leading-[22px] font-semibold">{children()}</h2> }
}
