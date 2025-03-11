use leptos::prelude::CustomAttribute;
use leptos::prelude::ElementChild;
use leptos::view;

#[leptos::component]
pub fn FocusIconInactive() -> impl leptos::IntoView {
    view! {
        <svg
            width="25"
            height="24"
            viewBox="0 0 25 24"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                d="M17.2778 2H18.8333C21.0425 2 22.8333 3.79086 22.8333 6V7.55556M8.38887 2H6.83331C4.62417 2 2.83331 3.79086 2.83331 6V7.55556M22.8333 16.4444V18C22.8333 20.2091 21.0425 22 18.8333 22H17.2778M8.38887 22H6.83331C4.62417 22 2.83331 20.2091 2.83331 18V16.4444M17.8333 12C17.8333 14.7614 15.5947 17 12.8333 17C10.0719 17 7.83331 14.7614 7.83331 12C7.83331 9.23858 10.0719 7 12.8333 7C15.5947 7 17.8333 9.23858 17.8333 12Z"
                stroke="#28303F"
                stroke-width="1.5"
                stroke-linecap="round"
            />
        </svg>
    }
}
