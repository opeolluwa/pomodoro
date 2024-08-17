import HomeIcon from "../../../public/icon/outlined/home.svg";
import FocusIcon from "../../../public/icon/outlined/focus.svg";
import ProfileIcon from "../../../public/icon/outlined/profile.svg";
import AnalyticsIcon from "../../../public/icon/outlined/analytics.svg";

import SolidHomeIcon from "../../../public/icon/solid/home.svg";
import SolidFocusIcon from "../../../public/icon/solid/focus.svg";
import SolidProfileIcon from "../../../public/icon/solid/profile.svg";
import SolidAnalyticsIcon from "../../../public/icon/solid/analytics.svg";

const routes = [
  {
    name: "Home",
    defaultIcon: HomeIcon,
    solidIcon: SolidHomeIcon,
    route: "/home",
  },
  {
    name: "Focus",
    defaultIcon: FocusIcon,
    solidIcon: SolidFocusIcon,
    route: "/focus",
  },
  {
    name: "Profile",
    defaultIcon: ProfileIcon,
    solidIcon: SolidProfileIcon,
    route: "/profile",
  },
  {
    name: "Analytics",
    defaultIcon: AnalyticsIcon,
    solidIcon: SolidAnalyticsIcon,
    route: "/analytics",
  },
];


export default function BottomNav() {
  return (
    <ul className="flex justify-between items-center w-full bg-white px-6">
      {routes.map((route) => (
        <li className="flex flex-col py-4 gap-2 hover:text-app border-t-2">
          <img src={route.defaultIcon} alt={route.name} />
          <a href={route.route}>{route.name}</a>
        </li>
      ))}
    </ul>
  );
}
