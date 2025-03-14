use crate::components::cards::generic_card::GenericCard;
use crate::components::typography::heading::HeadingText;
use crate::layouts::app_layout::AppLayout;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::RwSignal;
use leptos::prelude::Set;
use leptos::view;
use thaw::Button;
use thaw::Flex;
use thaw::FlexJustify;
use thaw::Select;
use thaw::Switch;

#[leptos::component]
pub fn ProfileScreen() -> impl leptos::IntoView {
    let header = view! { <HeadingText>Profile</HeadingText> };
    let input_box_css_rule = "border border-gray-200 border-2 rounded-lg w-16 h-10 placeholder:text-center placeholder:text-grap-400";
    let input_box_wrapper_css_rule = "flex flex-col justify-center items-center";
    let value = RwSignal::new("Red".to_string());
    let section_title_css_rule = "text-[#525772] leading-[16px] text-[14px] block uppercase small";
    view! {
        <AppLayout header class="h-[90vh] overflow scroll " active_route="profile">

            // TODO: only show if account exists
            <HeadingText>User Information</HeadingText>
            <GenericCard class="my-2 shadow-sm">
                <p>Jane doe</p>
                <p>example@mailer.com</p>
            </GenericCard>

            <HeadingText>Settings</HeadingText>
            <GenericCard class="my-2 shadow-sm">
                <span class=section_title_css_rule>TIMER</span>
                <small class="block">In minutes</small>
                <div class="bg-[#F7F7F7] py-[20px] flex justify-evenly mb-6 rounded ">
                    <span class=input_box_wrapper_css_rule>
                        <label>Counts</label>
                        <input placeholder="0" class=input_box_css_rule />
                    </span>
                    <span class=input_box_wrapper_css_rule>
                        <label>"Short break"</label>
                        <input placeholder="0" class=input_box_css_rule />

                    </span>
                    <span class=input_box_wrapper_css_rule>
                        <label>"Long break"</label>
                        <input placeholder="0" class=input_box_css_rule />
                    </span>
                </div>

                <Flex justify=FlexJustify::SpaceBetween class="my-3">
                    <span>Auto start breaks</span>
                    <Switch class=" rounded-full border-none" checked=true />
                </Flex>

                <Flex justify=FlexJustify::SpaceBetween class="my-3">
                    <span>Auto start pomodoro</span>
                    <Switch class="rounded-full border-none" checked=false />
                </Flex>

                <Flex justify=FlexJustify::SpaceBetween class="my-3 hidden">
                    <span>Counts before long break</span>
                    <input
                        placeholder="5"
                        class="order border-gray-200 border-2 rounded-lg size-10 placeholder:text-center placeholder:text-grap-400"
                    />
                </Flex>
            </GenericCard>

            <GenericCard class="my-2 shadow-sm">
                <span class=section_title_css_rule>Sound</span>

                <Flex justify=FlexJustify::SpaceBetween class="my-3">
                    <span>Alarm sound</span>
                    <Select value>
                        <option>"Red"</option>
                        <option>"Green"</option>
                        <option>"Blue"</option>
                    </Select>
                </Flex>

                <Flex justify=FlexJustify::SpaceBetween class="my-3">
                    <span>Auto start pomodoro</span>
                    <Switch class="rounded-full border-none" checked=false />
                </Flex>
            </GenericCard>

            <GenericCard class="my-2 shadow-sm">
                <span class=section_title_css_rule>NOTIFICATION</span>

                <Flex justify=FlexJustify::SpaceBetween class="my-3">
                    <span>Reminder</span>
                    <Switch class=" rounded-full border-none" checked=true />
                </Flex>

                <Flex justify=FlexJustify::SpaceBetween class="my-3">
                    <span>Connect with device alarm</span>
                    <Switch class="rounded-full border-none" checked=false />
                </Flex>
            </GenericCard>

            <GenericCard class="my-2 shadow-sm">
                <span class=section_title_css_rule>SYNC</span>
                <Flex justify=FlexJustify::SpaceBetween class="my-3">
                    <span>Sync with Google calendar</span>
                    <Switch class="rounded-full border-none" checked=false />
                </Flex>
            </GenericCard>
        </AppLayout>
    }
}
