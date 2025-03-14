use crate::components::cards::generic_card::GenericCard;
use crate::components::typography::heading::HeadingText;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::view;
use reactive_stores::Patch;
use reactive_stores_macro::Store;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Store, Debug, Serialize, Deserialize, Patch)]
pub struct NotificationOptions {
    pub heading: String,
    pub time: String,
    pub content: String,
    pub unread: bool,
    pub key: u8,
}
#[derive(Clone, Store, Debug, Serialize, Deserialize, Patch)]

pub struct ClientNofification {
    #[store(key: usize = |entry| entry.key.into())]
    pub notification: Vec<NotificationOptions>,
}

#[leptos::component]
pub fn NotificationCard(
    #[prop()] heading: String,
    #[prop()] time: String,
    #[prop()] content: String,
    #[prop()] unread: bool,
    #[prop(optional)] class: &'static str,

) -> impl leptos::IntoView {
    view! {
        <GenericCard class=class>
            <HeadingText class="first-letter:capitalize ">
                <span class="text-[#0D1C36]" class=("text-app-green", move || unread == true)>
                    {heading}
                </span>
            </HeadingText>
            <small>{time}</small>
            <p class="text-[#0D1C36] leading-[20px] py-[10px] font-[14px]">{content}</p>

        </GenericCard>
    }
}
