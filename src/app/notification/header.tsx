import Heading from "@/components/Heading";
import SmallText from "@/components/SmallText";
import View from "@/components/View";

export default function NotificationPageHeader() {
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
