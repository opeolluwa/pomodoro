use leptos::prelude::view;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;

use crate::icons::bell::BellIconOutline;

#[leptos::component]
pub fn HomeScreenHeader() -> impl leptos::IntoView {
    view! {
        <div class="flex items-center justify-between">
            <div>
                <small class="text-app-text text-[12px]" style="line-height:16px">
                    // TODO: calculete the hour of the day and gree accordingly
                    Good Afternoon
                </small>
                <h1 class="text-app-text font-bold" style="line-height:20px">
                    Scarlet Anderson
                </h1>
            </div>
            <BellIconOutline />
        </div>
    }
}
