use leptos::prelude::ClassAttribute;
use leptos::prelude::CustomAttribute;
use leptos::prelude::ElementChild;
use leptos::view;

#[leptos::component]
pub fn FocusIconInactive() -> impl leptos::IntoView {
    view! {
        <svg
            width="24"
            height="24"
            viewBox="0 0 25 24"
            fill="none"
            class="size-6"
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

#[leptos::component]
pub fn FocusIconActive() -> impl leptos::IntoView {
    view! {
        <svg
            width="25"
            height="24"
            viewBox="0 0 25 24"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                d="M21.5 17.9668V10.1503C21.5 8.93937 20.9604 7.7925 20.0301 7.02652L15.0301 2.90935C13.5577 1.69688 11.4423 1.69689 9.96986 2.90935L4.96986 7.02652C4.03964 7.7925 3.5 8.93937 3.5 10.1503V17.9668C3.5 20.1943 5.29086 22 7.5 22H17.5C19.7091 22 21.5 20.1943 21.5 17.9668Z"
                stroke="#28303F"
                stroke-width="1.5"
                stroke-linejoin="round"
            />
            <path d="M10.5 18H14.5" stroke="#28303F" stroke-width="1.5" stroke-linecap="round" />
        </svg>
    }
}
