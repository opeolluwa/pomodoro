import Heading from "@/components/Heading";
import View from "@/components/View";

export function ProfilePageHeader(): React.JSX.Element {
  return (
    <View className="flex items-center justify-between p-0 m-0">
      <View className="bg-white">
        <Heading className="text-[16px]">Profile</Heading>
      </View>
    </View>
  );
}
