use crate::components::typography::heading::HeadingText;
use crate::icons::focus::FocusScreenEmptyStateIcon;
use crate::layouts::app_layout::AppLayout;
use leptos::prelude::*;
use leptos::view;
use thaw::DrawerBody;
use thaw::DrawerPosition;
use thaw::OverlayDrawer;

#[leptos::component]
pub fn FocusScreen() -> impl leptos::IntoView {
    let header = view! { <HeadingText>Focus</HeadingText> };
    let open = RwSignal::new(false);

    view! {
        <AppLayout header class="h-[90vh] bg-white overflow-scroll " active_route="focus">

            <button class="text-white justify-center bg-app-green w-full flex items-center rounded px-4 py-3 gap-x-2 " on:click = move |_| open.set(true)>
                <svg
                    width="25"
                    height="24"
                    viewBox="0 0 25 24"
                    fill="none"
                    xmlns="http://www.w3.org/2000/svg"
                >
                    <path
                        d="M12.5 8V16M16.5 12H8.5M12.5 22C18.0228 22 22.5 17.5228 22.5 12C22.5 6.47715 18.0228 2 12.5 2C6.97715 2 2.5 6.47715 2.5 12C2.5 17.5228 6.97715 22 12.5 22Z"
                        stroke="white"
                        stroke-width="1.5"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    />
                </svg>
                <span class="text-[20px] font-medium">Add template</span>
            </button>

            <div class="flex flex-col justify-center items-center   h-[70vh]">
                <FocusScreenEmptyStateIcon />
            </div>

           <OverlayDrawer open position=DrawerPosition::Bottom>

        <DrawerBody>
          <p>"Drawer content"</p>
        </DrawerBody>
    </OverlayDrawer>
        </AppLayout>
    }
}
