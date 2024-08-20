import Heading from "@/components/Heading";
import View from "@/components/View";

export function AnalyticsPageHeader(): React.JSX.Element {
  return (
    <View className="flex items-center justify-between p-0 m-0">
      <View className="bg-white">
        <Heading className="text-[16px]">Analytics</Heading>
      </View>
    </View>
  );
}
