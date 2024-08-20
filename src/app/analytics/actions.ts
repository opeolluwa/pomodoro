export interface AnalyticActionInterface {
    text: String,
    exec?: () => void
    isActive: boolean
}

export const analyticsActions: AnalyticActionInterface[] = [
  {
      text: "Daily",
      isActive: false
  },
  {
      text: "Weekly",
      isActive: true
  },
  {
      text: "Monthly",
      isActive: false
  },
];
