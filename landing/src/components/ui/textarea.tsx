import * as React from "react";

import { cn } from "@/lib/utils";
import { Slot } from "@radix-ui/react-slot";
import { cva, VariantProps } from "class-variance-authority";

export const textareaVariants = cva(
  "w-full rounded-lg border border-gray-600 bg-[#2A2D40] px-20 py-12 text-white placeholder:text-gray-500 focus:outline-none focus:border-primary transition-colors resize-y min-h-[120px]",
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

export function Textarea({
  className,
  variant,
  asChild = false,
  ...props
}: React.ComponentProps<"textarea"> &
  VariantProps<typeof textareaVariants> & {
    asChild?: boolean;
  }) {
  const Comp = asChild ? Slot : "textarea";

  return (
    <Comp className={cn(textareaVariants({ variant, className }))} {...props} />
  );
}
