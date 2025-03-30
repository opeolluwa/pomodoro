
use js_bindgen::navigate::change_location_to;
use leptos::prelude::*;
use leptos::view;

use crate::components::typography::heading::HeadingText;
use crate::layouts::auth_layout::AuthenticationLayout;

#[leptos::component]
pub fn SetNewPasswordScreen() -> impl leptos::IntoView {
    let password_requirements = vec![
        "Minimum of 8 characters",
        "Include at least a special character",
        "Must contain letters and figures",
    ];
    view! {
        <AuthenticationLayout class="">

            <HeadingText class="text-center text-2xl">"Reset password"</HeadingText>

            
             <div class="flex  flex-col  gap-y-4 mt-8 ">

                <p class="prose-md mt-6">

                    Input your new password in the text box below, following this rules;

                </p>

                <ul>
                    {password_requirements
                        .into_iter()
                        .map(|entry| view! { <li class="text-left my-1">{entry}</li> })
                        .collect_view()}
                </ul>
              
   <input
                    type="text"
                    placeholder="Email"
                    class="input input-lg bg-[#f7f7f7] border-none hover:border-none focus:border-app-green/50 w-full "
                />

                
                <button
                    class="btn btn-lg bg-app-green text-white my-3   w-full border-none prose-lg"
                    on:click=move |_| change_location_to("/sign-up")
                >
         Save
                </button>

              
            </div>
        </AuthenticationLayout>
    }
}
