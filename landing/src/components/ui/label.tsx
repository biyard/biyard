import * as React from "react";

import { cn } from "@/lib/utils";
import { Slot } from "@radix-ui/react-slot";
import { cva, VariantProps } from "class-variance-authority";

export const labelVariants = cva(
  "font-medium text-[15px]/22 tracking-[0.5px] text-neutral-400",
  {
    variants: {
      variant: {
        default: "",
      },
    },
    defaultVariants: {
      variant: "default",
    },
  },
);

export function Label({
  className,
  variant,
  asChild = false,
  ...props
}: React.ComponentProps<"label"> &
  VariantProps<typeof labelVariants> & {
    asChild?: boolean;
  }) {
  const Comp = asChild ? Slot : "label";

  return (
    <Comp className={cn(labelVariants({ variant, className }))} {...props} />
  );
}
