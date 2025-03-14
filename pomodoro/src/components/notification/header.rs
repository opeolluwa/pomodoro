use crate::components::typography::heading::HeadingText;
use crate::icons::arrow::ArrowBack;
use js_bindgen::navigate::change_location_to;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::view;
use leptos::prelude::AddAnyAttr;
use leptos::prelude::IntoAttribute;

#[leptos::component]
pub fn NotificationScreenHeader() -> impl leptos::IntoView {
    view! {
        <div class="flex justify-between items-center">
            <h2 class="flex items-center gap-x-[10px]">
                <ArrowBack on:click=move |_| change_location_to("/") />
                <HeadingText>Notification</HeadingText>
            </h2>
            <button class="text-[#E06767] font-[14px]">Clear all</button>
        </div>
    }
}
