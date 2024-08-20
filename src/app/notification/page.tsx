"use client";

import Heading from "@/components/Heading";
import NotificationCard from "@/components/Notification";
import Text from "@/components/Text";
import View from "@/components/View";
import DashboardLayout from "@/layouts/DashboardLayout";
import { useState } from "react";
import NotificationPageHeader from "./header";
import { notifications } from "./notifications";



export default function NotificationPage() {
  const [notification, setNotification] = useState([]);
  if (!notifications || notifications.length === 0) {
    return (
      <DashboardLayout
        withBottomNav={false}
        headingSlot={<NotificationPageHeader />}
        className="flex flex-col items-center justify-center m-0 p-0"
      >
        <View className="flex flex-col items-center justify-center -mt-4">
          <Heading className="text-gray-600 text-[18px] ">No message !</Heading>
          <Text className="leading-7 text-center p-[14px] py-4 text-gray-400">
            My chief, I know you are doing your best focusing on your growth,
            but there is no message for you yet.
          </Text>
        </View>
      </DashboardLayout>
    );
  }

  return (
    <DashboardLayout
      withBottomNav={false}
      backArrow={false}
      headingSlot={<NotificationPageHeader/>}
      className="mb"
    >
      <View className="flex flex-col gap-y-4">
        {notifications.map((notification) => (
          <NotificationCard
            time={notification.time}
            title={notification.title}
            body={notification.body}
          ></NotificationCard>
        ))}
      </View>
    </DashboardLayout>
  );
}
