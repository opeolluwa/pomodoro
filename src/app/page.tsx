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
      <View>
        <h2 className="text-[14px] mb-2 font-bold text-gray-500">Quick Focus</h2>
        <Card className="bg-app px-12 py-6 rounded-sm min-h-[300px]">
          <ul className="flex justify-center items-center mb-6 text-[12px] text-white">
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
          <View className="min-h-[200px] w-[220px] mx-auto flex px-12 justify-center items-center bg-[rgba(245, 245, 245, 1)] border-[1px] my-4 rounded-sm border-white"></View>
          <Button className="bg-[#f7f7f7] flex justify-center items-center gap-2 w-4/5 mx-auto ">
            <PlayIcon className="w-6 h-6 text-app"></PlayIcon> Start
          </Button>
        </Card>
      </View>
    </DashboardLayout>
  );
}
