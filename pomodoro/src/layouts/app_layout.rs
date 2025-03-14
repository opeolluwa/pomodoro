use crate::icons::analytics::{AnalyticsIconActive, AnalyticsIconInactive};
use crate::icons::focus::{FocusIconActive, FocusIconInactive};
use crate::icons::home::{HomeIconActive, HomeIconInActive};
use crate::icons::profile::{ProfileIconActive, ProfileIconInactive};
use leptos::prelude::{ ClassAttribute, ElementChild, StyleAttribute};
use leptos::IntoView;
use leptos::{prelude::Children, view};

#[leptos::component]
pub fn AppLayout<F>(
    children: Children,
    header: F,
    #[prop(optional)] class: &'static str,
    #[prop(optional)] active_route: &'static str,
    #[prop(optional)] hide_dock: bool,
) -> impl leptos::IntoView
where
    F: IntoView,
{
    view! {
        <header class="relative">
            <nav class="px-[24px] py-[17px] bg-white top-10 bottom-20 left-0 w-full right-0 ">
                {header}
            </nav>
        </header>
        <main
            class=format!("bg-[#F5F5F5] py-[17px] pb-[10vh] overflow-y-scroll px-[24px] {}", class)
            style="position:relative"
        >

            {children()}
        </main>
        <nav
            class="px-[24px]  fixed w-full left-0 right-0 bottom-0 bg-[#FFFFFF] px-[20px] pb-[20px] flex items-center justify-between"
            class=("hidden", move || hide_dock == true)
        >

            // home
            <NavItem
                is_active=active_route == "home"
                label="Home"
                active_icon=HomeIconActive()
                inactive_icon=HomeIconInActive()
                path="/"
            />

            <NavItem
                is_active=active_route == "focus"
                label="Focus"
                active_icon=FocusIconActive()
                inactive_icon=FocusIconInactive()
                path="/focus"
            />

            <NavItem
                is_active=active_route == "analytics"
                label="Analytics"
                active_icon=AnalyticsIconActive()
                inactive_icon=AnalyticsIconInactive()
                path="/analytics"
            />

            <NavItem
                is_active=active_route == "profile"
                label="profile"
                active_icon=ProfileIconActive()
                inactive_icon=ProfileIconInactive()
                path="/profile"
            />
        </nav>
    }
}

#[leptos::component]
pub fn NavItem<A, I>(
    #[prop()] is_active: bool,
    #[prop()] active_icon: A,
    #[prop()] inactive_icon: I,
    #[prop()] label: &'static str,
    #[prop()] path: &'static str,
) -> impl leptos::IntoView
where
    A: IntoView,
    I: IntoView,
{
    let nav_item_css_rule =
        "flex flex-col items-center justify-start gap-y-[6.5px] hover:text-app-green pt-2";
    let nav_item_label_css_rule = "font-[12px] leading-[16px]";
    let active = move || is_active == true;
    let in_active = move || is_active == false;

    view! {
        <a
            class=nav_item_css_rule
            href=path
            class=(
                "border-t-2 border-t-solid border-t-2 border-t-app-green",
                move || is_active == true,
            )
        >
            <span class=("hidden", in_active)>{active_icon}</span>
            <span class=("hidden", active)>{inactive_icon}</span>
            <span class=nav_item_label_css_rule class=("text-app-green", active)>
                {label}
            </span>
        </a>
    }
}
