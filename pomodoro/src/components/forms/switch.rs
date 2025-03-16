use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::view;

#[leptos::component]
pub fn Switch(
    #[prop(optional)] class: &'static str,
    #[prop()] checked: bool,
) -> impl leptos::IntoView {
    view! {

<label class="inline-flex items-center mb-5 cursor-pointer">
  <input type="checkbox" value="" class="sr-only peer"/>
  <div class="relative w-9 h-5 bg-gray-300 peer-focus:outline-none  rounded-full peer  peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-4 after:w-4 after:transition-all peer-checked:bg-app-green "></div>
  
</label>

      }
}
