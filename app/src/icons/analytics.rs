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
                d="M16.3566 2H7.97663C4.33663 2 2.16663 4.17 2.16663 7.81V16.19C2.16663 19.83 4.33663 22 7.97663 22H16.3566C19.9966 22 22.1666 19.83 22.1666 16.19V7.81C22.1666 4.17 19.9966 2 16.3566 2ZM7.91663 13.6C7.91663 14.01 7.57663 14.35 7.16663 14.35C6.75663 14.35 6.41663 14.01 6.41663 13.6V10.4C6.41663 9.99 6.75663 9.65 7.16663 9.65C7.57663 9.65 7.91663 9.99 7.91663 10.4V13.6ZM12.9166 15.34C12.9166 15.75 12.5766 16.09 12.1666 16.09C11.7566 16.09 11.4166 15.75 11.4166 15.34V8.66C11.4166 8.25 11.7566 7.91 12.1666 7.91C12.5766 7.91 12.9166 8.25 12.9166 8.66V15.34ZM17.9166 13.6C17.9166 14.01 17.5766 14.35 17.1666 14.35C16.7566 14.35 16.4166 14.01 16.4166 13.6V10.4C16.4166 9.99 16.7566 9.65 17.1666 9.65C17.5766 9.65 17.9166 9.99 17.9166 10.4V13.6Z"
                fill="#75CE8E"
            />
        </svg>
    }
}
