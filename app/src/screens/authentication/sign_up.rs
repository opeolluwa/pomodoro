use leptos::prelude::*;
use leptos::view;

use crate::components::typography::heading::HeadingText;
use crate::layouts::auth_layout::AuthenticationLayout;

#[leptos::component]
pub fn SignUpScreen() -> impl leptos::IntoView {
    view! {
        <AuthenticationLayout class="">

            <HeadingText class="text-center text-2xl">"Sign up!"</HeadingText>

            <form class="flex items-center flex-col mt-12  gap-y-6">
                <input
                    type="text"
                    placeholder="full name"
                    class="input input-lg  bg-[#f7f7f7] border-none hover:border-none focus:border-app-green/50 w-full"
                />

                <input
                    type="text"
                    placeholder="email"
                    class="input input-lg bg-[#f7f7f7] border-none hover:border-none focus:border-app-green/50 w-full "
                />

                <input
                    type="text"
                    placeholder="occupation"
                    class="input input-lg bg-[#f7f7f7] border-none hover:border-none focus:border-app-green/50 "
                />

                <input
                    type="text"
                    placeholder="password"
                    class="input input-lg bg-[#f7f7f7] border-none hover:border-none focus:border-app-green/50"
                />

                <button class="shadow btn-lg flex items-center justify-center  btn bg-white text-[#525772] w-full border-none prose-lg">
                    <img src="public/images/google-icon.png" class="object-fit size-6" />
                    Continue with Google
                </button>

                <button class="btn btn-lg  bg-app-green text-white  w-full border-none prose-lg">
                    Continue
                </button>

            </form>

            <div class="prose mt-6 text-center text-gray-600">
                "Already have an account?" <a href="/" class="text-app-green no-underline pl-2">
                    Sign in
                </a>
            </div>
        </AuthenticationLayout>
    }
}
