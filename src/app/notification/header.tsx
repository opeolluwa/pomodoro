import Heading from "@/components/Heading";
import SmallText from "@/components/SmallText";
import View from "@/components/View";
import { ArrowLeftIcon } from "@heroicons/react/16/solid";

export default function NotificationPageHeader() {
  return (
    <>
      <View className="flex items-center justify-between p-0 m-0">
        <View className="flex items-center gap-x-2"> 
          <ArrowLeftIcon className="w-4 h-4 inline " />{" "}
          <Heading className="text-[16px]">Notification</Heading>
        </View>
        <SmallText className="text-red-400">Clear all</SmallText>
      </View>
    </>
  );
}
