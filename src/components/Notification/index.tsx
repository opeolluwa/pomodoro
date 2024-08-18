import React from "react";
import Card from "../Card";
import Heading from "../Heading";
import Text from "../Text";
import { NotificationInterface } from "@/app/notification";

export interface Props extends NotificationInterface {}

export default function NotificationCard({ time, title, body }: Props) {
  return (
    <>
      <Card className="bg-white px-4 py-2  shadow">
        <Heading className="text-[18px] first-letter:uppercase">{title}</Heading>
        <Text>{time}</Text>
        <Text className="leading-2 text-[14px] py-4 first-letter:uppercase text-[#0D1C36]">
          {body}
        </Text>
      </Card>
    </>
  );
}
