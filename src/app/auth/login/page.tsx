import Button from "@/components/Button";
import Heading from "@/components/Heading";
import SmallText from "@/components/SmallText";
import PageLayout from "@/layouts/PageLayout";
import React from "react";

export default function LoginPage() {
  return (
    <PageLayout className="flex flex-col ">
      <div>
        <Heading className="text-center text-[18px] font-bold">Sign In</Heading>
        <div className="flex justify-center items-center mt-12">
          <img
            src="../../assets/login-page-illustration.png"
            alt="login icon"
            className=" w-[180px]"
          />
        </div>
      </div>

      <div>
        <label htmlFor="email" className="sr-only">
          Email
        </label>
        <input
          type="email"
          className="form-input mt-4 px-4 rounded border-none outline-none w-full py-3 bg-[#f7f7f7]"
          placeholder="Email"
        />
      </div>

      <div>
        <label htmlFor="email" className="sr-only">
          Password
        </label>
        <input
          type="password"
          className="form-input mt-4 px-4 rounded border-none outline-none w-full py-3 bg-[#f7f7f7]"
          placeholder="Password"
        />
      </div>

      <Button className="shadow rounded-[5px] font-[20px] py-[15px] px-[20px] text-center text-neutral bg-white flex items-center justify-center gap-2 mt-6">
        <img src="../../assets/google-icon.png" alt="" className="w-[20px]" />
        <span> Continue with google</span>
      </Button>

      <Button
        href="/auth/login"
        className="shadow rounded-[5px] font-[20px] py-[15px] px-[20px] text-center text-[#f5f5f5] bg-[#75CE8E] w-full mt-6"
      >
        Continue
      </Button>

      <SmallText className="text-center mt-4">
        Have you{" "}
        <a className="text-app" href="/auth/forgotten-password">
          forgotten passord
        </a>
      </SmallText>
    </PageLayout>
  );
}
