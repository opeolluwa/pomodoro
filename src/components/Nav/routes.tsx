import {
  ChartBarSquareIcon,
  CubeIcon,
  HomeIcon,
  UserIcon,
} from "@heroicons/react/24/outline";

export default [
  {
    name: "Home",
    defaultIcon: <HomeIcon className="w-6 h-6" />,
    route: "/",
  },
  {
    name: "Focus",
    defaultIcon: <CubeIcon className="w-6 h-6" />,
    route: "/auth",
  },

  {
    name: "Analytics",
    defaultIcon: <ChartBarSquareIcon className="w-6 h-6" />,
    route: "/analytics",
  },

  {
    name: "Profile",
    defaultIcon: <UserIcon className="w-6 h-6" />,
    route: "/profile",
  },
];
