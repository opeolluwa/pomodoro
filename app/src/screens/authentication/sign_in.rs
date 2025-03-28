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

                <form class="flex flex-col gap-y-6 w-full mt-6">

                    <input
                        type="text"
                        placeholder="email"
                        class="input input-md bg-[#f7f7f7] border-none hover:border-none focus:border-app-green/50 w-full "
                    />

                    <input
                        type="text"
                        placeholder="password"
                        class="input input-md bg-[#f7f7f7] border-none hover:border-none focus:border-app-green/50"
                    />

                    <button class="shadow btn-md flex items-center justify-center my-3 btn bg-white text-[#525772] w-full border-none prose-lg">
                        <img src="public/images/google-icon.png" class="object-fit size-4" />
                        Continue with Google
                    </button>

                    <button class="btn btn-md  bg-app-green text-white my-3   w-full border-none prose-lg">
                        Continue
                    </button>
                </form>

                <span class="prose mt-6">
                    "Already have an account?"
                    <a href="/" class="text-app-green text-decoration-none pl-2">
                        Sign in
                    </a>
                </span>
            </div>
        </AuthenticationLayout>
    }
}
