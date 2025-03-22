use crate::components::focus::cards::FocusCardTemplate;
use crate::components::typography::heading::HeadingText;
use crate::icons::focus::FocusScreenEmptyStateIcon;
use crate::layouts::app_layout::AppLayout;
use crate::state::templates::FocusTemplateCardOptions;
use crate::state::templates::FocusTemplateKind;
use crate::state::templates::StoredTemplates;
use crate::state::templates::StoredTemplatesStoreFields;
use leptos::prelude::*;
use leptos::view;
use reactive_stores::Store;
use thaw::DrawerBody;
use thaw::DrawerHeader;
use thaw::DrawerPosition;
use thaw::DrawerSize;
use thaw::OverlayDrawer;

#[leptos::component]
pub fn FocusScreen() -> impl leptos::IntoView {
    let header = view! { <HeadingText>Focus</HeadingText> };
    let open = RwSignal::new(false);
    let input_box_css_rule = "border border-gray-200 border-2 rounded-lg w-16 h-10 placeholder:text-center placeholder:text-grap-400 outline-none text-center";
    let input_box_wrapper_css_rule = "flex flex-col justify-center items-center";

    let templates: Vec<FocusTemplateCardOptions> = vec![FocusTemplateCardOptions {
        kind: FocusTemplateKind::Fitness,
        title: "gg".to_string(),
        key: 1,
        description: "des".to_string(),
        // timer: FocusTemplateTimerConfig {
        //     count: 5,
        //     short_break: 5,
        //     long_break: 5,
        // },
    }];

    let store = Store::new(StoredTemplates { templates });

    let categories = vec!["fitness", "work", "study", "personal"];
    view! {
        <AppLayout header class="h-[90vh] bg-white overflow-scroll " active_route="focus">

            <button
                class="text-white justify-center bg-app-green w-full flex items-center rounded px-4 py-3 gap-x-2 "
                on:click=move |_| open.set(true)
            >
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

            <Show
                when=move || { store.templates().get().len() >= 1 as usize }
                fallback=|| {
                    view! {
                        <div class="flex flex-col justify-center items-center   h-[70vh]">
                            <FocusScreenEmptyStateIcon />
                        </div>
                    }
                }
            >
                <For
                    each=move || { store.templates().get() }
                    key=|template| template.key
                    let(entry)
                >
                    <FocusCardTemplate
                        kind=entry.kind
                        title=entry.title
                        description=entry.description
                    />
                // timer=entry.timer
                </For>
            </Show>

            <OverlayDrawer
                open
                position=DrawerPosition::Bottom
                size=DrawerSize::Medium
                class="rounded-t-lg"
            >
                <DrawerHeader>
                    <HeadingText>Create a template</HeadingText>
                </DrawerHeader>
                <DrawerBody class="h-[90vh] overflow-scroll">

                    <form class="flex flex-col gap-y-6 mt-6">

                        <div>
                            <label
                                for="message"
                                class="block mb-2 text-sm font-medium  text-[#525772]"
                            >
                                "What do you want to do?"
                            </label>

                            <input
                                placeholder="e.g, Assignment, web design..."
                                class="rounded bg-[#F7F7F7] outline-none border-none rounded placeholder:text-[#B2B6CB] w-full  px-4 py-3"
                            />
                        </div>

                        <div>

                            <label
                                for="message"
                                class="block mb-2 text-sm font-medium text-[#525772"
                            >
                                Description
                            </label>
                            <textarea
                                id="message"
                                rows="4"
                                class="block p-2.5 w-full text-sm text-gray-900 bg-[#f7f7f7] w-full border-none rounded-lg  focus:ring-app-green focus:border-app-green"
                                placeholder="Description"
                            ></textarea>

                        </div>

                        <div>
                            <label
                                for="message"
                                class="block mb-2 text-sm font-medium  text-[#525772]"
                            >
                                Category
                            </label>
                            <div class="flex items-center justify-between ">

                                {categories
                                    .into_iter()
                                    .map(|category| {
                                        view! {
                                            <button class="rounded hover:bg-app-green bg-[#f7f7f7] hover:text-white text-[#525772] py-2 px-3 capitalize">
                                                {category}
                                            </button>
                                        }
                                    })
                                    .collect_view()}
                            </div>

                        </div>
                        <div>
                            <label
                                for="message"
                                class="block mb-2 text-sm font-medium  text-[#525772]"
                            >
                                "Timer config"
                            </label>

                            <div class="bg-[#F7F7F7] py-[20px] flex justify-evenly rounded ">

                                <span class=input_box_wrapper_css_rule>
                                    <label>Counts</label>
                                    <input placeholder="0" class=input_box_css_rule />
                                </span>
                                <span class=input_box_wrapper_css_rule>
                                    <label class="text-[#0D1C36]">"Short break"</label>
                                    <input placeholder="0" class=input_box_css_rule />

                                </span>
                                <span class=input_box_wrapper_css_rule>
                                    <label>"Long break"</label>
                                    <input placeholder="0" class=input_box_css_rule />
                                </span>
                            </div>

                        </div>
                        <button
                            class="text-white justify-center bg-app-green w-full flex items-center rounded px-4 py-3 gap-x-2 text-[20px] font-medium"
                            on:click=move |_| open.set(false)
                        >

                            Save
                        </button>
                    </form>
                </DrawerBody>
            </OverlayDrawer>
        </AppLayout>
    }
}
