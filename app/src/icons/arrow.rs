use leptos::prelude::ClassAttribute;
use leptos::prelude::CustomAttribute;
use leptos::prelude::ElementChild;
use leptos::view;

#[leptos::component]
pub fn ArrowBack() -> impl leptos::IntoView {
    view! {
        <svg
            width="24"
            height="24"
            viewBox="0 0 24 24"
            class="size-6"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                d="M10 8L6 12M6 12L10 16M6 12L18 12"
                stroke="#0D1C36"
                stroke-width="1.5"
                stroke-linecap="round"
                stroke-linejoin="round"
            />
        </svg>
    }
}
