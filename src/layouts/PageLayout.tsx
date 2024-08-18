import React from "react";
interface Props {
  className?: string;
  children: React.ReactNode;
}
export default function PageLayout({ children, className }: Props) {
  
  return <div className={"px-6 bg-white py-10 min-h-screen " +className}>{children}</div>;
}
