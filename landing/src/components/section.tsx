import { ReactNode } from "react";

interface SectionProps {
  id: string;
  children: ReactNode;
  className?: string;
  containerClassName?: string;
}

export function Section({
  id,
  children,
  className = "",
  containerClassName = "",
}: SectionProps) {
  return (
    <section
      id={id}
      className={`overflow-hidden relative w-full flex justify-center scroll-mt-nav-offset ${className}`}
    >
      <div
        className={`flex flex-col w-full max-w-wrapper z-2 py-60 max-tablet:max-w-full max-tablet:px-40 ${containerClassName}`}
      >
        {children}
      </div>
    </section>
  );
}
