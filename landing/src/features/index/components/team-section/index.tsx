import { teamMembers } from "../../data";
import { MemberCard } from "./member-card";
import { Section } from "@/components/section";

export function TeamSection() {
  return (
    <Section id="our-team">
      <div className="absolute top-[1/2] -left-313 h-1261 w-1261 bg-purple-blur/40 blur-[500px]" />
      <div className="flex flex-col gap-48 w-full max-tablet:justify-center">
        <h2 className="font-medium text-center text-white text-[45px]/64">
          Our <span className="text-primary">Team</span>
        </h2>

        <div className="grid grid-cols-4 gap-24 w-full max-tablet:flex max-tablet:flex-row max-tablet:overflow-x-scroll scrollbar-hide">
          {teamMembers.map((member) => (
            <MemberCard member={member} />
          ))}
        </div>
      </div>
    </Section>
  );
}
