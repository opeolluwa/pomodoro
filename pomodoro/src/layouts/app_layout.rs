use std::fmt::format;

use leptos::prelude::StyleAttribute;
use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::IntoView;
use leptos::{prelude::Children, view};

use crate::icons::analytics::AnalyticsIconInactive;
use crate::icons::focus::FocusIconInactive;
use crate::icons::home::HomeIconActive;
use crate::icons::profile::ProfileIconInactive;

#[leptos::component]
pub fn AppLayout<F>(
    children: Children,
    header: F,
    #[prop(optional)] class: &'static str,
) -> impl leptos::IntoView
where
    F: IntoView,
{
    let nav_item_css_rule ="flex flex-col items-center justify-center gap-y-[5px] ";
    let nav_item_label_css_rule ="font-[12px] leading-[16px]";

    view! {
        <header class="px-[24px] py-[17px] bg-white">{header}</header>
        <main
            class=format!("bg-[#F5F5F5] py-[17px] overflow-y-scroll px-[24px] {}", class)
            style="position:relative"
        >
            {children()}
        </main>
        <nav class="px-[24px] dock fixed w-full left-0 right-0 bottom-0 bg-[#FFFFFF] px-[20px] pb-[20px] flex items-center justify-between pt-2">

            <button class=nav_item_css_rule>
                <HomeIconActive />
                 <span class=format!("text-app-green {}", nav_item_label_css_rule)>Home</span>
            </button>

            <button class=nav_item_css_rule>
                <FocusIconInactive />
                <span class=nav_item_label_css_rule>Focus</span>
            </button>

            <button class=nav_item_css_rule>
                <AnalyticsIconInactive />
                <span class=nav_item_label_css_rule>Analytics</span>
            </button>

            <button class=nav_item_css_rule>
                <ProfileIconInactive />
                <span class=nav_item_label_css_rule>Profile</span>
            </button>
        </nav>
    }
}
