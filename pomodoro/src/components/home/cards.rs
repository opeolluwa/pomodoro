use leptos::view;
use leptos::prelude::ElementChild;
use leptos::prelude::ClassAttribute;

use crate::components::cards::generic_card::GenericCard;
use crate::components::typography::heading::HeadingText;
use crate::components::typography::paragraph::BaseText;
use crate::icons::play::PlayIconGreen;

#[leptos::component]
pub fn TimerCard() -> impl leptos::IntoView {
    let timeslot_incative_css_rule =
        "text-[#F7F7F7] leading-[16px] font-[12px] px-[10px] py-[5px] ";
    let timeslot_ative_css_rule =
        "text-[#05595B] bg-[rgba(245, 245, 245, 0.5)] px-[10px] py-[5px] ";

    let timer_css_rule = "bg-[#F5F5F5] leading-[36px] py-[20px] px-[10px]rounded-[10px] w-[60px] h-[60px] flex flex-col justify-center items-center text-4xl rounded-[10px] font-semibold";
    view! {
        <div>
            <div class="mt-2 flex flex-col relative p-5 gap-5 min-h-[380px] bg-[#75CE8E] rounded-[10px] shadow-sm">
                <div class="flex items-center justify-center">
                    <button class=timeslot_incative_css_rule>Pomodoro</button>
                    <button class=timeslot_ative_css_rule>"Short break"</button>
                    <button class=timeslot_incative_css_rule>"Long break"</button>
                </div>

                // the time box
                <div class="bg-white/10 w-[220px] h-[220px] rounded-[5px] border-[1px] border-white mx-auto flex justify-center items-center gap-x-3 text-[#525772]">
                    <span class=timer_css_rule>00</span>
                    <span class="text-4xl font-bold leading-[36px]">:</span>
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
  view! {
      <GenericCard class="mt-[20px] min-h-[380px] shadow-sm">

          <HeadingText>Activity check</HeadingText>
          // TODO: compute the message and render activities if any
          <BaseText class="text-left">No recent activity</BaseText>

      </GenericCard>
  }
}