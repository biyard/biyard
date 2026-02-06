import { cn } from "@/lib/utils";

interface CardTitleProps {
  children: React.ReactNode;
  size?: "large" | "medium";
  weight?: "bold" | "semibold";
  className?: string;
}

const sizeStyles = {
  large: "text-xl leading-28",
  medium: "text-base leading-24",
};

const weightStyles = {
  bold: "font-bold",
  semibold: "font-semibold",
};

export function CardTitle({
  children,
  size = "large",
  weight = "semibold",
  className,
}: CardTitleProps) {
  return (
    <h3
      className={cn(
        "text-white font-outfit tracking-[0.25px]",
        sizeStyles[size],
        weightStyles[weight],
        className
      )}
    >
      {children}
    </h3>
  );
}
