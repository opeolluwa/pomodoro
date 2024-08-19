interface Props {
  children?: React.ReactNode;
  className?: string;
  href?: string;
}

export default function Button({ children, className, href }: Props) {
  if (href?.trim()){
    return (
      <a href={href} className={"rounded px-4 py-3  no-underline" + className}>
        {children}
      </a>
    );
  }
  return (
    <button className={"rounded border-none outline-none px-4 py-3 " + className}>{children}</button>
  );
}
