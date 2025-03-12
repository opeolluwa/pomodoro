use leptos::prelude::ClassAttribute;
use leptos::prelude::CustomAttribute;
use leptos::prelude::ElementChild;
use leptos::view;
#[leptos::component]
pub fn AnalyticsIconInactive() -> impl leptos::IntoView {
    view! {
        <svg
            width="24"
            height="24"
            viewBox="0 0 26 26"
            fill="none"
            class="size-6"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                d="M6.85089 19.3158L6.85089 14.2632"
                stroke="#2A353D"
                stroke-width="1.5"
                stroke-linecap="round"
            />
            <path
                d="M13.1667 19.3158L13.1667 6.6842"
                stroke="#2A353D"
                stroke-width="1.5"
                stroke-linecap="round"
            />
            <path
                d="M19.4825 19.3158L19.4825 11.7368"
                stroke="#2A353D"
                stroke-width="1.5"
                stroke-linecap="round"
            />
            <path
                d="M1.16669 13C1.16669 7.34315 1.16669 4.51472 2.92405 2.75736C4.68141 1 7.50983 1 13.1667 1C18.8235 1 21.652 1 23.4093 2.75736C25.1667 4.51472 25.1667 7.34315 25.1667 13C25.1667 18.6569 25.1667 21.4853 23.4093 23.2426C21.652 25 18.8235 25 13.1667 25C7.50983 25 4.68141 25 2.92405 23.2426C1.16669 21.4853 1.16669 18.6569 1.16669 13Z"
                stroke="#2A353D"
                stroke-width="1.5"
                stroke-linejoin="round"
            />
        </svg>
    }
}

#[leptos::component]
pub fn AnalyticsIconActive() -> impl leptos::IntoView {
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
