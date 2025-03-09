use leptos::prelude::ElementChild;
use leptos::{prelude::ClassAttribute, view};

use crate::icons::bell::BellIconOutline;
use crate::layouts::app_layout::AppLayout;

#[leptos::component]
pub fn HomeScreen() -> impl leptos::IntoView {
    let header = view! {
        <div class="flex items-center justify-between py-[17px]">
            <div>
                <small class="text-[#525772] text-[12px]" style="line-height:16px">
                    // TODO: calculete the hour of the day and gree accordingly
                    Good Afternoon
                </small>
                <h1 class="text-[#525772] font-bold" style="line-height:20px">
                    Scarlet Anderson
                </h1>
            </div>
            <BellIconOutline />
        </div>
    };
    view! {
        <AppLayout header class="h-[90vh]">
            hey man
        </AppLayout>
    }
}
