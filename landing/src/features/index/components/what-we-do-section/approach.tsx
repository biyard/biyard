import React from "react";

interface ApproachProps {
  title: string;
  description: string;
  children?: React.ReactNode;
}

export function Approach({ title, description, children }: ApproachProps) {
  return (
    <div className="flex flex-col col-span-1 gap-8 justify-start items-start max-tablet:items-center max-tablet:justify-center">
      {children}

      <div className="flex flex-col gap-12 justify-start items-start max-tablet:items-center max-tablet:justify-center">
        <h2 className="font-semibold text-white text-[28px]/36">{title}</h2>
        <p className="font-extralight text-gray-300 text-[15px]/23">
          {description}
        </p>
      </div>
    </div>
  );
}
