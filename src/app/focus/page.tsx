import React from "react";
import DashboardLayout from "@/layouts/DashboardLayout";
import { FocusPageHeader } from "./header";
import View from "@/components/View";

export default function Focuspage() {
  return (
    <DashboardLayout headingSlot={<FocusPageHeader />} withBottomNav={true}>
      <View></View>
    </DashboardLayout>
  );
}
