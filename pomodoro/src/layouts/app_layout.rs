

use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::prelude::{Show, StyleAttribute};
use leptos::IntoView;
use leptos::{prelude::Children, view};

use crate::icons::analytics::{AnalyticsIconActive, AnalyticsIconInactive};
use crate::icons::focus::{FocusIconActive, FocusIconInactive};
use crate::icons::home::{HomeIconActive, HomeIconInActive};
use crate::icons::profile::{ProfileIconActive, ProfileIconInactive};

#[leptos::component]
pub fn AppLayout<F>(
    children: Children,
    header: F,
    #[prop(optional)] class: &'static str,
    active_route: &'static str,
) -> impl leptos::IntoView
where
    F: IntoView,
{
    let nav_item_css_rule =
        "flex flex-col items-center justify-center gap-y-[5px] hover:text-app-green";

         let nav_item_css_rule =
        "flex flex-col items-center justify-center gap-y-[5px] hover:text-app-green";
    let nav_item_label_css_rule = "font-[12px] leading-[16px]";


    view! {
        <header class="relative">
            <nav class="px-[24px] py-[17px] bg-white fied top-10 bottom-20 left-0 w-full right-0">
                {header}
            </nav>
        </header>
        <main
            class=format!("bg-[#F5F5F5] py-[17px] overflow-y-scroll px-[24px] {}", class)
            style="position:relative"
        >
        {active_route}
            {children()}
        </main>
        <nav class="px-[24px]  fixed w-full left-0 right-0 bottom-0 bg-[#FFFFFF] px-[20px] pb-[20px] flex items-center justify-between pt-2">

        //home
            <Show
                when=move || active_route == "home"
                fallback=move || {
                    view! {
                        <a class=nav_item_css_rule href="/">
                            <HomeIconInActive />
                            <span class=format!(
                                "text-app-green {}",
                                nav_item_label_css_rule,
                            )>Home</span>
                        </a>
                    }
                }
            >

                <a class=nav_item_css_rule href="/">
                    <HomeIconActive />
                    <span class=format!("text-app-green {}", nav_item_label_css_rule)>Home</span>
                </a>
            </Show>

            // focus
            <Show
                when=move || active_route == "focus"
                fallback=move || {
                    view! {
                        <a class=nav_item_css_rule href="/focus">
                            <FocusIconInactive />
                            <span class=nav_item_label_css_rule>Focus</span>
                        </a>
                    }
                }
            >
                <a class=nav_item_css_rule href="/focus">
                    <FocusIconActive />
                    <span class=nav_item_label_css_rule>Focus</span>
                </a>
            </Show>

            // analytics
            <Show
                when=move || active_route == "analytics"
                fallback=move || {
                    view! {
                        <a class=nav_item_css_rule href="/analytics">
                            <AnalyticsIconInactive />
                            <span class=nav_item_label_css_rule>Analytics</span>
                        </a>
                    }
                }
            >
                <a class=nav_item_css_rule href="/profile">
                    <AnalyticsIconActive />
                    <span class=nav_item_label_css_rule>Analytics</span>
                </a>
            </Show>

            // profile

            <Show
                when=move || active_route == "home"
                fallback=move || {
                    view! {
                        <a class=nav_item_css_rule href="/profile">
                            <ProfileIconInactive />
                            <span class=nav_item_label_css_rule>Profile</span>
                        </a>
                    }
                }
            >
                <a class=nav_item_css_rule href="/profile">
                    <ProfileIconActive />
                    <span class=nav_item_label_css_rule>Profile</span>
                </a>
            </Show>
        </nav>
    }
}
