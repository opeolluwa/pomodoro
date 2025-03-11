use crate::components::typography::heading::HeadingText;
use crate::components::typography::paragraph::BaseText;
use crate::{icons::arrow::ArrowBack, layouts::app_layout::AppLayout};
use js_bindgen::prev_location;
use leptos::prelude::AddAnyAttr;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::IntoAttribute;
use leptos::view;

#[leptos::component]
pub fn NotificationScreen() -> impl leptos::IntoView {
    let header = view! {
        <div class="flex justify-between items-center">
            <h2 class="flex items-center gap-x-[10px]">
                <ArrowBack on:click= move |_| prev_location::go_to_prev_location() />

                <HeadingText>Notification</HeadingText>
            </h2>
            <button class="text-[#E06767] font-[14px]">Clear all</button>
        </div>
    };

    view! {
        <AppLayout header>
            <div class="flex flex-col items-center justify-center h-[90vh] ">
                <HeadingText>"No message!"</HeadingText>
                <BaseText class="text-center">
                    "My chief, I know you are doing your best focusing on your growth, but there is no message for you yet."
                </BaseText>
            </div>
        </AppLayout>
    }
}
