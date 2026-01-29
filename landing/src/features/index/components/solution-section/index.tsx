import { Section } from "@/components/section";
import { TrendingUp, Shield, Target, Flag } from "lucide-react";

const solutions = [
  {
    id: 1,
    icon: TrendingUp,
    title: "Higher probability of sustainable protection",
    description:
      "Revenue-linked reserves and clear downside protection make it easier for serious capital and communities to support your launch.",
  },
  {
    id: 2,
    icon: Shield,
    title: "Reduction in operational and regulatory risk",
    description:
      "Automated treasury rules, on-chain audits and DAO-based approval create a defensible risk posture for your team.",
  },
  {
    id: 3,
    icon: Target,
    title: "Less infra, more product focus",
    description:
      "Use our audited smart contracts, dashboards and integrations, without co-marketing your own DeFi stack.",
  },
  {
    id: 4,
    icon: Flag,
    title: "Fast idea to revenue-backed launch",
    description:
      "Standardized modules let you launch in days, not months, while still aligning with institutional-grade expectations.",
  },
];

export function SolutionSection() {
  return (
    <Section
      id="solution"
      containerClassName="flex-col items-center gap-48 max-tablet:gap-32"
    >
      <div className="flex flex-col items-center gap-16 text-center">
        <p className="text-primary text-sm font-semibold uppercase">SOLUTION</p>
        <h2 className="font-bold text-[30px]/[42px] max-tablet:text-[24px]/[32px]">
          Quantifiable impact{" "}
          <span className="text-primary">for your launch and treasury</span>
        </h2>
      </div>

      <div className="grid grid-cols-2 gap-24 w-full max-tablet:grid-cols-1 max-w-800">
        {solutions.map((solution) => (
          <SolutionCard
            key={solution.id}
            icon={solution.icon}
            title={solution.title}
            description={solution.description}
          />
        ))}
      </div>
    </Section>
  );
}

interface SolutionCardProps {
  icon: React.ComponentType<{ size?: number; className?: string }>;
  title: string;
  description: string;
}

function SolutionCard({ icon: Icon, title, description }: SolutionCardProps) {
  return (
    // <div className="flex flex-col gap-16 items-start p-32 rounded-2xl border border-white/10 backdrop-blur-sm hover:border-primary/30 transition-all bg-linear-to-br from-white/5 via-white/5 to-primary/5">
    <div
      className="flex flex-col gap-16 items-start p-32
      bg-white/3 backdrop-blur-sm
      rounded-2xl border border-white/10
      bg-linear-to-br from-transparent/5 via-transparent to-white/4"
    >
      <div className="size-32 flex items-center justify-center">
        <Icon size={24} className="text-primary" />
      </div>
      <h3 className="text-white text-xl font-semibold font-outfit leading-28 tracking-[0.25px]">
        {title}
      </h3>
      <p className="text-gray-400 text-sm font-light font-outfit leading-20 tracking-[0.25px]">
        {description}
      </p>
    </div>
  );
}
