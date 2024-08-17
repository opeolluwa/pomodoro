import Button from "@/components/Button";
import Heading from "@/components/Heading";
import SmallText from "@/components/SmallText";
import PageLayout from "@/layouts/PageLayout";
import React from "react";

export default function LoginPage() {
  return (
    <PageLayout className="flex flex-col ">
      <div>
        <Heading className="text-center text-[18px] font-bold">
          Reset password
        </Heading>
      </div>

      <div className="flex flex-col  gap-y-4 mt-12">
        <p className="leading-7  p-[14px] py-4">
          Please type in your email, we will send you a link to reset the
          password.
        </p>
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
        <div>
          <label htmlFor="password" className="sr-only">
          Confirm  Password
          </label>
          <input
            type="password"
            className="form-input mt-4 px-4 rounded border-none outline-none w-full py-3 bg-[#f7f7f7]"
            placeholder="Confirm assword"
          />
        </div>
      </div>

      <Button
        href="/auth/confirm-reset-password"
        className="shadow rounded-[5px] font-[20px] py-[15px] px-[20px] text-center text-[#f5f5f5] bg-[#75CE8E] w-full mt-6"
      >
        Continue
      </Button>
    </PageLayout>
  );
}
