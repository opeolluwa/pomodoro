import Button from "@/components/Button";
import PageLayout from "@/layouts/PageLayout";
import React from "react";

export default function WelcomePage() {
  return (
    <PageLayout className="bg-white">
      <h1 className="font-bold leading-[22px] text-center text-[18px]">
        Welcome!
      </h1>
      <div className="flex justify-center items-center mt-24">
        <img
          src="../../assets/welcome-page-illustration.png"
          alt=""
          className="p-4 mt-12 w-[180px]"
        />
      </div>
      <p className="leading-8 text-center p-[14px] py-4">
        Increase your productivity and manage your time effectively with
        Pomodore.
      </p>
      <div className="flex flex-col gap-8">
        <Button
          href="/auth/login"
          className="shadow rounded-[5px] font-[20px] py-[15px] px-[20px] text-center text-neutral bg-[#f5f5f5]"
        >
          Sign In
        </Button>

        <Button
          href="/auth/login"
          className="shadow rounded-[5px] font-[20px] py-[15px] px-[20px] text-center text-[#f5f5f5] bg-[#75CE8E]"
        >
          Sign Up
        </Button>

        <Button className="shadow rounded-[5px] font-[20px] py-[15px] px-[20px] text-center text-neutral bg-white flex items-center justify-center gap-2">
          <img src="../../assets/google-icon.png" alt="" className="w-[24px]" />
          <span> Continue with Google</span>
        </Button>
      </div>
    </PageLayout>
  );
}
