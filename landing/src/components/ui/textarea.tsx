import * as React from "react";

import { cn } from "@/lib/utils";
import { Slot } from "@radix-ui/react-slot";
import { cva, VariantProps } from "class-variance-authority";

export const textareaVariants = cva(
  "w-full rounded-[4px] border-1 border-gray-600 px-16 py-11 flex flex-row items-center focus:outline-none focus:border-primary",
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
