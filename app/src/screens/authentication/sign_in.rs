use js_bindgen::navigate::change_location_to;
use leptos::prelude::*;
use leptos::view;

use crate::components::typography::heading::HeadingText;
use crate::layouts::auth_layout::AuthenticationLayout;

#[leptos::component]
pub fn SignInScreen() -> impl leptos::IntoView {
    view! {
        <AuthenticationLayout class="">

            <HeadingText class="text-center text-2xl">"Sign in"</HeadingText>

            <div class="flex  flex-col justify-center items-center  h-[90vh]">
                <img
                    src="public/images/sign-up-page-illustration.png"
                    class="object-contain h-40 block"
                />

                <form class="flex flex-col gap-y-8 w-full mt-6">

                    <input
                        type="text"
                        placeholder="Email"
                        class="input input-lg bg-[#f7f7f7] border-none hover:border-none focus:border-app-green/50 w-full "
                    />

                    <input
                        type="text"
                        placeholder="Password"
                        class="input input-lg bg-[#f7f7f7] border-none hover:border-none focus:border-app-green/50"
                    />

                    <button class="shadow btn-lg flex items-center justify-center btn bg-white text-[#525772] w-full border-none prose-lg">
                        <img src="public/images/google-icon.png" class="object-fit size-4" />
                        Continue with Google
                    </button>

                    <button class="btn btn-lg  bg-app-green text-white w-full border-none ">
                        Continue
                    </button>
                </form>

                <span class="prose mt-6">
                    "Have you"
                    <a href="/reset-password" class="text-app-green no-underline pl-2">
                    forgotten password
                    </a>
                </span>
            </div>
        </AuthenticationLayout>
    }
}
