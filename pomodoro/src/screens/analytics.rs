use crate::components::cards::generic_card::GenericCard;
use crate::components::profile::typography::SectionTitle;
use crate::components::typography::heading::HeadingText;
use crate::components::typography::paragraph::BaseText;
use crate::layouts::app_layout::AppLayout;
use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild, StyleAttribute};
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

                <Flex
                    align=FlexAlign::Center
                    justify=FlexJustify::SpaceBetween
                    class=" w-[90%] mx-auto mt-6 "
                >
                    <div
                        class="rounded-[8px] py-[10px] bg-[#75CE8ECC]"
                        style="width:105px;
                        height:98px;
                        border-radius: 8px;
                        gap: 10px;
                        padding: 10px;
                        "
                    >
                        <svg
                            width="26"
                            height="26"
                            viewBox="0 0 26 26"
                            fill="none"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <path
                                d="M11.8971 14.3822L8.19995 6.99982L14.7035 12.1354C15.6281 12.8655 15.6328 14.2564 14.7132 14.9927C13.7936 15.7289 12.4227 15.4318 11.8971 14.3822Z"
                                stroke="#05595B"
                                stroke-width="1.5"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            />
                            <path
                                d="M4.6 4.38383C2.37837 6.56666 1 9.60845 1 12.9729C1 19.6153 6.37258 25 13 25C19.6274 25 25 19.6153 25 12.9729C25 7.14887 20.8697 2.2917 15.3847 1.18328C14.3815 0.980556 13.8799 0.879192 13.4399 1.24009C13 1.601 13 2.18441 13 3.35122V4.55393"
                                stroke="#05595B"
                                stroke-width="1.5"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            />
                        </svg>

                    </div>

                    <div
                        class="rounded-[8px] py-[10px] bg-[#75CE8ECC]"
                        style="width:105px;
                        height:98px;
                        border-radius: 8px;
                        gap: 10px;
                        padding: 10px;
                        "
                    >
                        <svg
                            width="24"
                            height="24"
                            viewBox="0 0 24 24"
                            fill="none"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <path
                                d="M12 23C16.1421 23 19.5 19.6421 19.5 15.5C19.5 14.6345 19.2697 13.8032 19 13.0296C17.3333 14.6765 16.0667 15.5 15.2 15.5C19.1954 8.5 17 5.5 11 1.5C11.5 6.49951 8.20403 8.77375 6.86179 10.0366C5.40786 11.4045 4.5 13.3462 4.5 15.5C4.5 19.6421 7.85786 23 12 23ZM12.7094 5.23498C15.9511 7.98528 15.9666 10.1223 13.463 14.5086C12.702 15.8419 13.6648 17.5 15.2 17.5C15.8884 17.5 16.5841 17.2992 17.3189 16.9051C16.6979 19.262 14.5519 21 12 21C8.96243 21 6.5 18.5376 6.5 15.5C6.5 13.9608 7.13279 12.5276 8.23225 11.4932C8.35826 11.3747 8.99749 10.8081 9.02477 10.7836C9.44862 10.4021 9.7978 10.0663 10.1429 9.69677C11.3733 8.37932 12.2571 6.91631 12.7094 5.23498Z"
                                fill="#124226"
                            />
                        </svg>

                    </div>
                </Flex>
            </GenericCard>
        </AppLayout>
    }
}
