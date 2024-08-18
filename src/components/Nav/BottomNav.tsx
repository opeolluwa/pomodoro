import routes from "./routes";

export default function BottomNav() {
  return (
    <ul className="flex justify-between items-center w-full bg-white px-6">
      {routes.map((route) => (
        <li className="flex flex-col justify-center items-center py-4 gap-y-1 hover:text-app ">
          {route.defaultIcon}
          <a href={route.route} className="text-[12px]">{route.name}</a>
        </li>
      ))}
    </ul>
  );
}
