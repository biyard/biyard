import { cn } from "@/lib/utils";

interface SectionTitleProps {
  children: React.ReactNode;
  size?: "large" | "medium" | "small";
  weight?: "black" | "bold" | "medium";
  align?: "left" | "center" | "right";
  className?: string;
}

const sizeStyles = {
  large: "text-[45px]/64 max-tablet:text-[32px]/42",
  medium: "text-[36px]/[48px] max-tablet:text-[28px]/[38px]",
  small: "text-[30px]/[42px] max-tablet:text-[24px]/[32px]",
};

const weightStyles = {
  black: "font-black",
  bold: "font-bold",
  medium: "font-medium",
};

const alignStyles = {
  left: "text-left",
  center: "text-center",
  right: "text-right",
};

export function SectionTitle({
  children,
  size = "small",
  weight = "bold",
  align = "left",
  className,
}: SectionTitleProps) {
  return (
    <h2
      className={cn(
        "text-white",
        sizeStyles[size],
        weightStyles[weight],
        alignStyles[align],
        className
      )}
    >
      {children}
    </h2>
  );
}
