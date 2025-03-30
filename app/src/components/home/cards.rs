use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::view;

use crate::components::cards::generic_card::GenericCard;
use crate::components::typography::heading::HeadingText;
use crate::components::typography::paragraph::BaseText;
use crate::icons::play::PlayIconGreen;

#[leptos::component]
pub fn TimerCard() -> impl leptos::IntoView {
    let timeslot_css_rule =
        "text-[#F7F7F7] leading-[16px] font-[12px] px-[10px] py-[5px] bg-dark hover:bg-black/30 rounded-sm  hover:text-white hover:bg-transparent";

    let timer_css_rule = "bg-[#F5F5F5] leading-[36px] py-[20px] px-[10px]rounded-[10px] w-[60px] h-[60px] flex flex-col justify-center items-center text-4xl rounded-[10px] font-semibold";
    view! {
        <div>
            <div class="mt-2 flex flex-col relative p-5 gap-5 min-h-[380px] bg-[#75CE8E] rounded-[10px] shadow-sm">
                <div class="flex items-center justify-center">
                    <button class=timeslot_css_rule>"Focus suration"</button>
                    <button class=timeslot_css_rule>"Break"</button>
                    <button class=timeslot_css_rule>"Count"</button>
                </div>

                // the time box
                <div class="bg-white/10 w-[220px] h-[220px] rounded-[5px] border-[1px] border-white mx-auto flex justify-center items-center gap-x-3 text-[#525772]">
                    <span class=timer_css_rule>25</span>
                    <span class="text-4xl font-black text-white font-bold leading-[36px] text">
                        :
                    </span>
                    <span class=timer_css_rule>00</span>
                </div>

                <button class="flex flex-row justify-center items-center px-5 py-3 gap-2 h-[54px] bg-[#F7F7F7] shadow-md rounded-md">
                    <PlayIconGreen />
                    <span class="h-5 text-[20px] leading-5 font-normal text-[#525772] flex items-center">
                        Start
                    </span>
                </button>
            </div>

        </div>
    }
}

#[leptos::component]
pub fn ActivityCard() -> impl leptos::IntoView {
    // spawn_local(async move {
    //     let database = database::load_database().await;
    //     let quotes = database
    //         .execute("SELECT * FROM quotes LIMIT 1", vec![])
    //         .await;

    //     let parsed_quotes: Vec<Quotes> = serde_wasm_bindgen::from_value(quotes).unwrap();
    //     set_quotes.set(parsed_quotes);
    // });

    // let q = quotes.get();

    view! {
        <GenericCard class="mt-[20px] min-h-[380px] shadow-sm">
            <HeadingText>Activity check</HeadingText>
            // TODO: compute the message and render activities if any
            <BaseText class="text-left mt-2">No recent activity</BaseText>

        // <div class="w-full text-white itallic py-4 px-4" style="
        // background: linear-gradient(90.71deg, rgba(255, 146, 239, 0) -5.82%, #92FFC0 -5.82%, #002661 100.87%);
        // border-radius: 5px;
        // ">
        // {quote}
        // </div>
        </GenericCard>
    }
}
