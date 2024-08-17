import Button from "@/components/Button";
import Heading from "@/components/Heading";
import SmallText from "@/components/SmallText";
import PageLayout from "@/layouts/PageLayout";
import React from "react";

export default function LoginPage() {
  return (
    <PageLayout className="flex flex-col justify-center g-4 items-center ">
      <img
        src="../../assets/account-verified-illustration.png"
        alt="login icon"
        className=" w-[100px]"
      />

      <p className="leading-7 text-center p-[14px] py-4">
        Your account has been verified successfully
      </p>
    </PageLayout>
  );
}
