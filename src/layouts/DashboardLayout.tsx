import Heading from "@/components/Heading";
import BottomNav from "@/components/Nav/BottomNav";
import View from "@/components/View";
import { ArrowLeftIcon } from "@heroicons/react/24/solid";
import React from "react";
interface Props {
  className?: string;
  children: React.ReactNode;
  backArrow?: boolean;
  headingText?: string;
  headingSlot: React.JSX.Element;
  withBottomNav: boolean;
  headerStyles?: String;
}
export default function PageLayout({
  children,
  className,
  backArrow,
  headingText,
  headingSlot,
  headerStyles,
  withBottomNav,
}: Props) {
  const heading = headingSlot ? headingSlot : headingText ? headingText : "";
  return (
    <View className="relative">
      <View className={"py-6 px-6  bg-white " + headerStyles}>
        {backArrow ? <ArrowLeftIcon className="w-6 h-6 inline " /> : ""}{" "}
        <Heading className="p-0 m-0">
          {heading ? <span className="">{heading}</span> : ""}
        </Heading>
      </View>
      <View className={"px-6 py-4 bg-[#f5f5f5] min-h-screen mb-4 " + className}>
        {children}
      </View>
      <View className={withBottomNav ? "fixed bottom-0 w-full left-0" : "hidden"}>
        {withBottomNav ? <BottomNav></BottomNav> : ""}
      </View>
    </View>
  );
}
