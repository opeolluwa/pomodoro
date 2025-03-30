

use js_bindgen::navigate::change_location_to;
use leptos::prelude::*;
use leptos::view;

use crate::components::typography::heading::HeadingText;
use crate::layouts::auth_layout::AuthenticationLayout;

#[leptos::component]
pub fn ResetPasswordScreen() -> impl leptos::IntoView {
    view! {
        <AuthenticationLayout class="">

            <HeadingText class="text-center text-2xl">"Reset password"</HeadingText>

            <div class="flex  flex-col justify-center items-center gap-y-4 mt-8 ">

                <p class="prose-md mt-6">

                    Please type in your email, we will send you a link to reset the password.
                </p>

                <input
                    type="text"
                    placeholder="Email"
                    class="input input-lg bg-[#f7f7f7] border-none hover:border-none focus:border-app-green/50 w-full "
                />

                <a
                    href="/confirm-reset-otp"
                    class="btn btn-lg bg-app-green text-white my-3   w-full border-none text-lg "
                >

                    Continue
                </a>

            </div>
        </AuthenticationLayout>
    }
}
