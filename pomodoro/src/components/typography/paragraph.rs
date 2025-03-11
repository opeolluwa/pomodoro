use leptos::prelude::view;
use leptos::prelude::Children;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::StyleAttribute;

#[leptos::component]
pub fn BaseText(
    children: Children,
    #[prop(optional)] class: &'static str,
) -> impl leptos::IntoView {
    view! {
        <p
          
            class=format!("text-[#B2B6CB] leading-[30px] text-center {class} px-6")
        >
            {children()}
        </p>
    }
}
