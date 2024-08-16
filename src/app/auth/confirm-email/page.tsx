"use client";

import Button from "@/components/Button";
import Heading from "@/components/Heading";
import SmallText from "@/components/SmallText";
import PageLayout from "@/layouts/PageLayout";
import { GetProps, Input } from "antd";
import React from "react";

type OTPProps = GetProps<typeof Input.OTP>;

export default function LoginPage() {
  const onChange: OTPProps["onChange"] = (text) => {
    console.log("onChange:", text);
  };

  const sharedProps: OTPProps = {
    onChange,
    className:"rounded-lg hidden bg-red-500"
  };
  return (
    <PageLayout className="flex flex-col ">
      <div>
        <Heading className="text-center text-[18px] font-bold">Sign In</Heading>
        <div className="flex justify-center items-center mt-24">
          <img
            src="../../assets/confirm-password-illustration.png"
            alt="login icon"
            className=" w-[100px]"
          />
        </div>
      </div>

      <p className="leading-7 text-center p-[14px] py-4 my-10">
        A message has been sent to your mail containing a four digit code, enter
        the code below to confirm email.
      </p>
      <div className="flex flex-col w-2/3  mx-auto justify-center items-center">
        <Input.OTP length={4} {...sharedProps} className="rounded-lg " />
      </div>
      <SmallText className="text-center mt-10 mb-4">
        Code expires in{" "}
        <a className="text-app" href="/auth/forgotten-password">
          00:22
        </a>
      </SmallText>

      <Button
        href="/auth/login"
        className="shadow rounded-[5px] font-[20px] py-[15px] px-[20px] text-center text-neutral bg-[#f5f5f5]"
      >
       Resend Code
      </Button>
    </PageLayout>
  );
}
