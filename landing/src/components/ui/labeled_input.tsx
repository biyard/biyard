import * as React from "react";

import { cn } from "@/lib/utils";
import { cva, VariantProps } from "class-variance-authority";
import { Input } from "./input";
import { Label } from "./label";

export const labeledInputVariants = cva("", {
  variants: {
    variant: {
      default: "class1",
    },
  },
  defaultVariants: {
    variant: "default",
  },
});

export function LabeledInput({
  className,
  variant,
  labelTitle,
  asChild = false,
  ...props
}: React.ComponentProps<"input"> &
  VariantProps<typeof labeledInputVariants> & {
    asChild?: boolean;
    labelTitle: string;
  }) {
  return (
    <div className="flex flex-col gap-8 items-start w-full">
      <Label>{labelTitle}</Label>
      <Input
        className={cn(labeledInputVariants({ variant, className }))}
        {...props}
      />
    </div>
  );
}
