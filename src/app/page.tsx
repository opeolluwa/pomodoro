import Button from "@/components/Button";
import Card from "@/components/Card";
import Heading from "@/components/Heading";
import SmallText from "@/components/SmallText";
import View from "@/components/View";
import DashboardLayout from "@/layouts/DashboardLayout";
import { BellIcon } from "@heroicons/react/24/outline";
import { PlayIcon } from "@heroicons/react/24/solid";
import React from "react";

export function HomePageHeader(): React.JSX.Element {
  return (
    <View className="flex items-center justify-between p-0 m-0">
      <View>
        <SmallText className="font-normal -mb-1">Good afternoon</SmallText>
        <Heading className="text-[16px]">Scarlet Anderson</Heading>
      </View>
      <Button href="/notification ">
        <BellIcon className="w-6 h-6"></BellIcon>
      </Button>
    </View>
  );
}

export default function HomePage() {
  const actions = [
    {
      title: "Pomodoro",
      isActive: true,
    },
    {
      title: "Short break",
      isAvtive: false,
    },
    {
      title: "Long break",
      isActive: false,
    },
  ];
  return (
    <DashboardLayout headingSlot={HomePageHeader()} withBottomNav={true}>
      <View className="mt-4">
        <h2 className="text-[14px] mb-1 font-bold">Quick Focus</h2>
        <Card className="bg-app px-12 py-6 rounded min-h-[300px]">
          <ul className="flex justify-center items-center mb-6 text-[12px]">
            {actions.map((action) => (
              <li
                className={
                  action.isActive
                    ? "bg-[#f5f5f5] text-[#05595B] px-2 py-1 "
                    : "px-2 py-1 color-white text-[12px]"
                }
              >
                {action.title}
              </li>
            ))}
          </ul>
          <View className="min-h-[200px] flex px-12 justify-center items-center bg-[rgba(245, 245, 245, .75)] border-2 my-4 rounded border-white"></View>
          <Button className="bg-[#f7f7f7] flex justify-center items-center gap-2 w-full">
            <PlayIcon className="w-6 h-6 text-app"></PlayIcon> Start
          </Button>
        </Card>
      </View>
    </DashboardLayout>
  );
}
