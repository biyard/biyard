import { cn } from "@/lib/utils";

interface SectionLabelProps {
  children: React.ReactNode;
  className?: string;
}

export function SectionLabel({ children, className }: SectionLabelProps) {
  return (
    <p
      className={cn(
        "text-primary text-sm font-semibold uppercase tracking-[2px]",
        className
      )}
    >
      {children}
    </p>
  );
}
