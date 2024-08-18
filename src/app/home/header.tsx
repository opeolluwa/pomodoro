import Heading from "@/components/Heading";
import SmallText from "@/components/SmallText";
import View from "@/components/View";
import { BellIcon } from "@heroicons/react/24/outline";
import { Button } from "antd";

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
