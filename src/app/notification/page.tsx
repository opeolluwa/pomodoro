"use client";

import Heading from "@/components/Heading";
import SmallText from "@/components/SmallText";
import Text from "@/components/Text";
import View from "@/components/View";
import DashboardLayout from "@/layouts/DashboardLayout";
import { useState } from "react";
import { notifications } from "./notifications";
import NotificationCard from "@/components/Notification";
import { time } from "console";

function NotificationPageHeader() {
  return (
    <>
      <View className="flex items-center justify-between p-0 m-0">
        <View>
          <Heading className="text-[16px]">Notification</Heading>
        </View>
        <SmallText className="text-red-500">Clear all</SmallText>
      </View>
    </>
  );
}

export default function NotificationPage() {
  const [notification, setNotification] = useState([]);
  if (!notification) {
    return (
      <DashboardLayout
        withBottomNav={true}
        headingSlot={NotificationPageHeader()}
      >
        <View>
          <Heading className="text-gray-600">No message !</Heading>
          <Text className="leading-8 text-center p-[14px] py-4 text-gray-400">
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
      backArrow={true}
      headingSlot={NotificationPageHeader()}
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
