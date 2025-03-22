use leptos::prelude::CustomAttribute;
use leptos::prelude::ElementChild;
use leptos::view;

#[leptos::component]
pub fn PlayIconGreen() -> impl leptos::IntoView {
    view! {
        <svg
            width="25"
            height="25"
            viewBox="0 0 25 25"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                d="M20.1834 12.9329L9.14311 20.2932C8.90378 20.4527 8.58041 20.388 8.42084 20.1487C8.36381 20.0632 8.33337 19.9626 8.33337 19.8598V5.13949C8.33337 4.85184 8.56656 4.61865 8.85421 4.61865C8.95703 4.61865 9.05756 4.64909 9.14311 4.70613L20.1834 12.0663C20.4226 12.2259 20.4873 12.5492 20.3278 12.7886C20.2896 12.8458 20.2406 12.8948 20.1834 12.9329Z"
                fill="#75CE8E"
            />
        </svg>
    }
}
