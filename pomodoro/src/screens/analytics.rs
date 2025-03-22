use crate::components::cards::generic_card::GenericCard;
use crate::components::profile::typography::SectionTitle;
use crate::components::typography::heading::HeadingText;
use crate::components::typography::paragraph::BaseText;
use crate::layouts::app_layout::AppLayout;
use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::view;
use thaw::{Flex, FlexAlign, FlexJustify};

#[leptos::component]
pub fn AnalyticsScreen() -> impl leptos::IntoView {
    let header = view! { <HeadingText>Analytics</HeadingText> };

    let button_inactive_css_rule =
        "bg-[#F7F7F7] text-[#525772] leading-[20px] text-center py-2 px-3 rounded";

    let button_active_css_rule =
        "bg-app-green text-white rounded leading-[20px] text-center py-2 px-3";
    view! {
        <AppLayout header class="h-[90vh] overflow scroll " active_route="analytics">

            <GenericCard>
                <SectionTitle>Productivity</SectionTitle>
                <BaseText>
                    Charts shows length of time spent per category in daily, weekly and monthly time frame.
                </BaseText>

                <Flex
                    align=FlexAlign::Center
                    justify=FlexJustify::SpaceBetween
                    class=" w-[90%] mx-auto mt-4 "
                >

                    <button class=button_inactive_css_rule>"Daily"</button>
                    <button class=button_active_css_rule>"Weekly"</button>
                    <button class=button_inactive_css_rule>"Monthly"</button>

                </Flex>

                <div class=" w-[90%] mx-auto my-6 rounded-[10px] border-[#E6E8F0]  shadow border-[2px] "></div>
            </GenericCard>
        </AppLayout>
    }
}
