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

                <p class="prose-lg mt-3 mb-4 text-center text-gray-600">

                    Increase your productivity and manage your time effectively with Pomodore.

                </p>

                <a
                    href="/sign-in"
                    class="btn btn-lg bg-[#f5f5f5] text-[#525772] mt-5 mb-3 w-full border-none text-lg shadow-sm"
                >

                    Sign in
                </a>

                <a
                    href="sign-up"
                    class="btn btn-lg bg-app-green text-white my-3   w-full border-none prose-lg shadow-sm"
                >
                    Sign up
                </a>

                <button class="shadow-sm flex items-center justify-center my-3 btn bg-white btn-lg text-[#525772] w-full border-none prose-lg font-base font-[400]">
                    <img src="public/images/google-icon.png" class="object-fit size-6" />
                    Continue with Google
                </button>
            </div>
        </AuthenticationLayout>
    }
}
