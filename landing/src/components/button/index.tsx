import { cn } from "@/lib/utils";
import { ButtonHTMLAttributes, forwardRef } from "react";

interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: "primary" | "outline";
  size?: "small" | "medium" | "large";
  shape?: "rounded" | "pill";
  fullWidth?: boolean;
}

const variantStyles = {
  primary:
    "text-black bg-[linear-gradient(93.06deg,#00D190_0%,#A9B5F3_99.39%)] shadow-[0px_20px_50px_20px_rgba(0,230,165,0.15)] hover:opacity-90 transition-opacity",
  outline:
    "text-white border border-white/20 hover:bg-white/10 transition-colors",
};

const sizeStyles = {
  small: "px-20 py-10 text-base",
  medium: "px-40 py-12 text-base",
  large: "px-20 py-15 text-lg",
};

const shapeStyles = {
  rounded: "rounded-xl",
  pill: "rounded-[50px]",
};

export const Button = forwardRef<HTMLButtonElement, ButtonProps>(
  (
    {
      children,
      variant = "primary",
      size = "medium",
      shape = "rounded",
      fullWidth = false,
      className,
      ...props
    },
    ref,
  ) => {
    return (
      <button
        ref={ref}
        className={cn(
          "flex flex-row justify-center items-center gap-10 font-semibold whitespace-nowrap",
          variantStyles[variant],
          sizeStyles[size],
          shapeStyles[shape],
          fullWidth && "w-full",
          "max-tablet:hidden",
          className,
        )}
        {...props}
      >
        {children}
      </button>
    );
  },
);

Button.displayName = "Button";
