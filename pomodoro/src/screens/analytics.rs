use crate::components::cards::generic_card::GenericCard;
use crate::components::profile::typography::SectionTitle;
use crate::components::typography::heading::HeadingText;
use crate::components::typography::paragraph::BaseText;
use crate::layouts::app_layout::AppLayout;
use leptos::prelude::{ClassAttribute, ElementChild, Signal};
use leptos::view;
use leptos_chartistry::*;
use thaw::{Flex, FlexAlign, FlexJustify};

pub struct MyData {
    x: f64,
    y1: f64,
    y2: f64,
}

#[leptos::component]
pub fn AnalyticsScreen() -> impl leptos::IntoView {
    let header = view! { <HeadingText>Analytics</HeadingText> };

    let series = Series::new(|data: &MyData| data.x)
        .bar(|data: &MyData| data.y1)
        .bar(|data: &MyData| data.y2);

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

pub fn load_data() -> Signal<Vec<MyData>> {
    Signal::derive(|| {
        vec![
            MyData::new(4.0, 5.0, 3.0),
            MyData::new(3.0, 2.5, 4.0),
            MyData::new(2.0, 2.25, 9.0),
            MyData::new(1.0, 3.0, 5.0),
        ]
    })
}

impl MyData {
    fn new(x: f64, y1: f64, y2: f64) -> Self {
        Self { x, y1, y2 }
    }
}
