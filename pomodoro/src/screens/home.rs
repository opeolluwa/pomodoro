use leptos::prelude::ElementChild;
use leptos::prelude::StyleAttribute;
use leptos::{prelude::ClassAttribute, view};

use crate::components::home::header::HomeScreenHeader;
use crate::icons::play::PlayIconGreen;
use crate::layouts::app_layout::AppLayout;

#[leptos::component]
pub fn HomeScreen() -> impl leptos::IntoView {
    let header = HomeScreenHeader();
    view! {
        <AppLayout header class="h-[90vh]">
            <h2 class="text-app-text leading-[14px] font-[600]">

                Quick Focus
            </h2>
            <div>
                <div class="mt-2 flex flex-col relative p-5 gap-5 min-h-[380px] bg-[#75CE8E] rounded-md">
                    <div class="flex items-center jsutify-center">
                        <button class="">Pomodoro</button>
                        <button>"Short break"</button>
                        <button>"Long break"</button>
                    </div>

                    <button class="flex flex-row justify-center items-center px-5 py-3 gap-2 h-[54px] bg-[#F7F7F7] shadow-md rounded-md">
                        <PlayIconGreen />

                        <span style="/* Label */
                        
                        height: 20px;    
                        /* Button/big */
                        font-family: 'Inter';
                        font-style: normal;
                        font-weight: 400;
                        font-size: 20px;
                        line-height: 20px;
                        /* identical to box height, or 100% */
                        display: flex;
                        align-items: center;
                        
                        /* text/subdued */
                        color: #525772;
                        
                        
                        /* Inside auto layout */
                        flex: none;
                        order: 1;
                        flex-grow: 0;
                        ">Start</span>
                    </button>
                </div>

            </div>
        </AppLayout>
    }
}
