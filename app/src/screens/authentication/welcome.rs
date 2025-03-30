
use js_bindgen::navigate::change_location_to;
use leptos::prelude::*;
use leptos::view;

use crate::components::typography::heading::HeadingText;
use crate::layouts::auth_layout::AuthenticationLayout;

#[leptos::component]
pub fn WelcomeScreen() -> impl leptos::IntoView {
    view! {
        <AuthenticationLayout class="">

            <HeadingText class="text-center text-2xl">"Welcome!"</HeadingText>

            <div class="flex  flex-col justify-center items-center  h-[90vh]">
                <img
                    src="/public/images/welcome-page-illustration.png"
                    class="object-contain h-40 block"
                />

                <p class="prose-md mt-3 text-center">

                    Increase your productivity and manage your time effectively with Pomodore.
                </p>
                <button
                    class="btn btn-lg bg-[#f5f5f5] text-[#525772] mt-5 mb-3 w-full border-none text-lg "
                    on:click=move |_| change_location_to("/sign-in")
                >
                    Sign in
                </button>

                <button
                    class="btn btn-lg bg-app-green text-white my-3   w-full border-none prose-lg"
                    on:click=move |_| change_location_to("/sign-up")
                >
                    Sign up
                </button>

                <button class="shadow flex items-center justify-center my-3 btn bg-white btn-lg text-[#525772] w-full border-none prose-lg">
                    <img src="public/images/google-icon.png" class="object-fit size-6" />
                    Continue with Google
                </button>
            </div>
        </AuthenticationLayout>
    }
}
