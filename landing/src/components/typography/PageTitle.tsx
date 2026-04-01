import { cn } from "@/lib/utils";

interface PageTitleProps {
  children: React.ReactNode;
  variant?: "hero" | "large";
  className?: string;
  align?: "left" | "center" | "right";
}

const variantStyles = {
  hero: "font-black uppercase text-[60px]/[66px] max-tablet:text-[44px]/[52px]",
  large: "font-medium text-[45px]/64",
};

const alignStyles = {
  left: "text-left",
  center: "text-center",
  right: "text-right",
};

export function PageTitle({
  children,
  variant = "hero",
  align = "left",
  className,
}: PageTitleProps) {
  return (
    <h1
      className={cn(
        "text-white",
        variantStyles[variant],
        alignStyles[align],
        className
      )}
    >
      {children}
    </h1>
  );
}
