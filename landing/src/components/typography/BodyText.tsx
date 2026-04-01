import { cn } from "@/lib/utils";

interface BodyTextProps {
  children: React.ReactNode;
  size?: "large" | "medium" | "small";
  color?: "default" | "light";
  className?: string;
  as?: "p" | "span" | "div";
}

const sizeStyles = {
  large: "text-[16px]/[24px] leading-24",
  medium: "text-[14px]/[20px]",
  small: "text-sm leading-20",
};

const colorStyles = {
  default: "text-gray-400",
  light: "text-gray-300",
};

export function BodyText({
  children,
  size = "medium",
  color = "default",
  className,
  as: Component = "p",
}: BodyTextProps) {
  return (
    <Component
      className={cn(
        "font-light font-outfit tracking-[0.25px]",
        sizeStyles[size],
        colorStyles[color],
        className
      )}
    >
      {children}
    </Component>
  );
}
