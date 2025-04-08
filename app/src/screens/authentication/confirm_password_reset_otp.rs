use leptos::prelude::*;
use leptos::view;

use crate::components::typography::heading::HeadingText;
use crate::layouts::auth_layout::AuthenticationLayout;

#[leptos::component]
pub fn ConfirmResetOtpScreen() -> impl leptos::IntoView {
    view! {
        <AuthenticationLayout class="">

            <HeadingText class="text-center text-2xl">"Confirm email"</HeadingText>

            <form class="flex  flex-col justify-center items-center gap-y-6  h-[90vh]">
                <img
                    src="/public/images/confirm-otp-illustration.png"
                    class="object-contain h-[125px] block"
                />

                <p class="prose-lg text-gray-600 mt-3 text-center">

                    A message has been sent to your mail containing a four digit code, enter the code below to reset your password.
                </p>

                <div class="flex items-center justify-center gap-x-6">
                    <input class="rounded-lg hover:bg-[#E6E8F0] focus:bg-[#E6E8F0] size-12  p-2 border-[2.22px] border-[#E6E8F0]" />
                    <input class="rounded-lg hover:bg-[#E6E8F0] focus:bg-[#E6E8F0] size-12  p-2 border-[2.22px] border-[#E6E8F0]" />
                    <input class="rounded-lg hover:bg-[#E6E8F0] focus:bg-[#E6E8F0] size-12  p-2 border-[2.22px] border-[#E6E8F0]" />
                    <input class="rounded-lg hover:bg-[#E6E8F0] focus:bg-[#E6E8F0] size-12  p-2 border-[2.22px] border-[#E6E8F0]" />

                </div>

                <div>
                    <span>Code expires in</span>
                    <span class="text-app-green pl-2">"0:22"</span>
                </div>
                <a
                    href="/set-new-password"
                    class="btn btn-lg bg-[#f5f5f5] text-[#525772] mt-5 mb-3 w-full border-none text-lg "
                >
                    Resend code
                </a>

            </form>
        </AuthenticationLayout>
    }
}
