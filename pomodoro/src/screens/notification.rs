use crate::components::notification::cards::ClientNofification;
use crate::components::notification::cards::ClientNofificationStoreFields;
use crate::components::notification::cards::NotificationCard;
use crate::components::notification::cards::NotificationOptions;
use crate::components::notification::header::NotificationScreenHeader;
use crate::components::typography::heading::HeadingText;
use crate::components::typography::paragraph::BaseText;
use crate::layouts::app_layout::AppLayout;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;

use leptos::prelude::For;
use leptos::prelude::Get;
use leptos::prelude::Show;
use leptos::view;
use reactive_stores::Store;

#[leptos::component]
pub fn NotificationScreen() -> impl leptos::IntoView {
    let header = NotificationScreenHeader();
    let hide_dock = true;

    let notifications = vec![NotificationOptions {
        heading: "Notification title".to_owned(),
        time: "now".to_string(),
        content: "Hello there, what have you been up to lately? We miss you on pomodore. Avoid distractions and focused today!".to_string(),
        unread: true, key:1
    }, NotificationOptions { heading: "notification title".to_string(), time: "2d ago".to_string(), content: "Hello there, what have you been up to lately? We miss you on pomodore. Avoid distractions and focused today!".to_string(), unread: false, key :2 },
    NotificationOptions { heading: "notification title".to_string(), time: "2d ago".to_string(), content: "Hello there, what have you been up to lately? We miss you on pomodore. Avoid distractions and focused today!".to_string(), unread: false, key :3 },
    NotificationOptions { heading: "notification title".to_string(), time: "2d ago".to_string(), content: "Hello there, what have you been up to lately? We miss you on pomodore. Avoid distractions and focused today!".to_string(), unread: false, key :4 }
    ];

    let store = Store::new(ClientNofification {
        notification: notifications,
    });

    view! {
        <AppLayout hide_dock header active_route="notification" class="h-[90vh] overflow-scroll ">
            <div
                class="flex flex-col"
                class=(
                    "justify-center items-center ",
                    move || store.notification().get().len() == 0,
                )
            >
                <Show
                    fallback=|| {
                        view! {
                            <HeadingText>"No message!"</HeadingText>
                            <BaseText class="text-center">
                                "My chief, I know you are doing your best focusing on your growth, but there is no message for you yet."
                            </BaseText>
                        }
                    }
                    when=move || { store.notification().get().len() >= 1 as usize }
                >
                    <For each=move || store.notification().get() key=|entry| entry.key let(entry)>
                        <NotificationCard
                            time=entry.time
                            heading=entry.heading
                            unread=entry.unread
                            content=entry.content
                            class="my-[10px] first:mt-0 last:mb-0"
                        />
                    </For>

                </Show>
            </div>
        </AppLayout>
    }
}
