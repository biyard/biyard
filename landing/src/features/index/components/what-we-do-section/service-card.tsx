import React from "react";
import { ArrowRight } from "@/components/icons";

interface ServiceCardProps {
  title: string;
  description: string;
  to?: string;
  children?: React.ReactNode;
}

export function ServiceCard({
  title,
  description,
  to,
  children,
}: ServiceCardProps) {
  return (
    <div className="flex flex-col col-span-1 items-start p-32 w-full border border-gray-800 bg-black/50 rounded-[16px] gap-54">
      <div className="flex flex-col gap-12 items-start w-full">
        <h2 className="font-semibold text-white text-[28px]/36">{title}</h2>
        <p className="font-extralight text-gray-300 text-[15px]/23">
          {description}
        </p>
      </div>

      <div className="flex flex-col gap-24 items-start w-full">
        {to ? (
          <a
            className="flex flex-row gap-4 h-24 font-semibold text-white text-base/16 tracking-[0.5px]"
            href={to}
          >
            Explore the service
            <ArrowRight
              className="[&>path]:stroke-white"
              height={18}
              width={18}
            />
          </a>
        ) : (
          <span
            className="flex flex-row gap-4 h-24 font-semibold text-white opacity-0 pointer-events-none text-base/16 tracking-[0.5px]"
            aria-hidden="true"
          >
            Explore the service
            <ArrowRight
              className="[&>path]:stroke-white"
              height={18}
              width={18}
            />
          </span>
        )}

        <div className="flex justify-center items-center w-full">
          {children}
        </div>
      </div>
    </div>
  );
}
