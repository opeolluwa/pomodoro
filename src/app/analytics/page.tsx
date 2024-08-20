"use client";

import React from "react";
import DashboardLayout from "@/layouts/DashboardLayout";
import { AnalyticsPageHeader } from "./header";
import View from "@/components/View";
import Heading from "@/components/Heading";
import Text from "@/components/Text";
import Card from "@/components/Card";
import { analyticsActions } from "./actions";
import Button from "@/components/Button";
import { ClockIcon, FireIcon } from "@heroicons/react/24/outline";

export default function AnalyticsPage() {
  return (
    <DashboardLayout headingSlot={<AnalyticsPageHeader />} withBottomNav={true}>
      <View className="bg-white px-6 py-6 rounded-md">
        <Heading className="text-[14px]">Productivity</Heading>
        <Text className="leading-6 text-[14px] px-0  text-gray-400">
          Charts shows length of time spent per category in daily, weekly and
          monthly time frame.
        </Text>
        <Card>
          <ul className="flex justify-center items-center gap-4">
            {analyticsActions.map((action) => (
              <li onClick={() => action.exec} className="text-[12px]">
                <Button
                  className={
                    action.isActive
                      ? "bg-app text-white px-2 py-2"
                      : "bg-[#f7f7f7] text-[#525772] px-2 py-2"
                  }
                >
                  {action.text}
                </Button>
              </li>
            ))}
          </ul>
          <View className="grid grid-cols-2 items-center justify-center gap-x-4  my-8">
            <Card className="bg-app-400  col-span-1 rounded shadow  p-2">
              <ClockIcon className="w-6 h-6 text-[#0D1C36]" />
              <Heading className="text-white text-[16px] mt-3 ">
                {" "}
                0 hours{" "}
              </Heading>
              <Text className="text-[#0D1C36] text-[14px] ">Hours Focused</Text>
            </Card>
            <Card className="bg-app-400 col-span-1 rounded shadow  p-2">
              <FireIcon className="w-6 h-6 text-[#0D1C36]" />
              <Heading className="text-white text-[14px] mt-3 ">
                {" "}
                0 days{" "}
              </Heading>
              <Text className="text-[#0D1C36]">Streak</Text>
            </Card>
          </View>
          <View>bars here</View>
        </Card>
      </View>
       <View className="bg-white px-6 py-6 rounded-md mt-4">
        <Card>
          <Heading className="text-[14px]">Achievement</Heading>
          <ul>
            <li>
              <Card></Card>
            </li>
          </ul>
        </Card>
      </View>
    </DashboardLayout>
  );
}
