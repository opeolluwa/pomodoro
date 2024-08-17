import Button from "@/components/Button";
import Heading from "@/components/Heading";
import SmallText from "@/components/SmallText";
import PageLayout from "@/layouts/PageLayout";
import React from "react";

export default function LoginPage() {
  return (
    <PageLayout className="flex flex-col ">
      <div>
        <Heading className="text-center text-[18px] font-bold">Sign up</Heading>
      </div>

      <div className="flex flex-col  gap-y-4 my-10">
        <div>
          <label htmlFor="full name" className="sr-only">
            Full name
          </label>
          <input
            type="text"
            className="form-input mt-4 px-4 rounded border-none outline-none w-full py-3 bg-[#f7f7f7]"
            placeholder="Full name"
          />
        </div>

        <div>
          <label htmlFor="email" className="sr-only">
         Email
          </label>
          <input
            type="email"
            className="form-input mt-4 px-4 rounded border-none outline-none w-full py-3 bg-[#f7f7f7]"
            placeholder="Password"
          />
        </div>

        <div>
          <label htmlFor="occupation" className="sr-only">
            Occupation
          </label>
          <input
            type="text"
            className="form-input mt-4 px-4 rounded border-none outline-none w-full py-3 bg-[#f7f7f7]"
            placeholder="What do you do?"
          />
        </div>

        <div>
          <label htmlFor="password" className="sr-only">
            Password
          </label>
          <input
            type="password"
            className="form-input mt-4 px-4 rounded border-none outline-none w-full py-3 bg-[#f7f7f7]"
            placeholder="Password"
          />
        </div>
      </div>

      <Button className="shadow rounded-[5px] font-[20px] py-[15px] px-[20px] text-center text-neutral bg-white flex items-center justify-center gap-2 mt-6">
        <img src="../../assets/google-icon.png" alt="" className="w-[20px]" />
        <span> Continue with google</span>
      </Button>

      <Button
        href="/auth/confirm-email"
        className="shadow rounded-[5px] font-[20px] py-[15px] px-[20px] text-center text-[#f5f5f5] bg-[#75CE8E] w-full mt-6"
      >
        Continue
      </Button>

      <SmallText className="text-center mt-4">
        Already have an account{" "}
        <a className="text-app" href="/auth/login">
          Sign in
        </a>
      </SmallText>
    </PageLayout>
  );
}
