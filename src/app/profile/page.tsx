import React from "react";
import DashboardLayout from "@/layouts/DashboardLayout";
import { ProfilePageHeader } from "./header";
import Card from "@/components/Card";
import Heading from "@/components/Heading";
import View from "@/components/View";
import Text from "@/components/Text";
import SmallText from "@/components/SmallText";
import Button from "@/components/Button";
import { ArrowLeftEndOnRectangleIcon } from "@heroicons/react/24/outline";
export default function ProfilePage() {
  return (
    <DashboardLayout withBottomNav={true} headingSlot={<ProfilePageHeader />}>
      <View>
        <Heading>User Information</Heading>
        <Card className="bg-white rounded-md my-2 grid grid-cols-2">
          <Text className="col-span-1">Adeoye Adefemi</Text>{" "}
          <Text className="col-span-1"> Full Stack engineer</Text>
          <SmallText className="col-span-2">adefemiadeoye@yahoo.com</SmallText>
        </Card>
      </View>
      <View className="mt-4">
        <Heading>Settings</Heading>
        <Card className="bg-white rounded-md my-4">
          <Text>TIMER</Text>
          <SmallText>In minutes</SmallText>
        </Card>

        <Card className="bg-white rounded-md my-4">
          <Text>SOUND</Text>
        </Card>

        <Card className="bg-white rounded-md my-2">
          <Text>NOTIFICATION</Text>
        </Card>

        <Card className="bg-white rounded-md my-2">
          <Text>SYNC</Text>
        </Card>
      </View>
      <Button
        href="/auth/login"
        className="mt-6 flex gap-2 items-center justify-center bg-app-400 text-white"
      >
        <ArrowLeftEndOnRectangleIcon className="w-4 h-4" /> Login to
        back-up-date
      </Button>
    </DashboardLayout>
  );
}
