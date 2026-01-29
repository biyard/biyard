import { Section } from "@/components/section";

const steps = [
  {
    id: 1,
    step: "STEP 1",
    title: "Connect your revenue",
    description:
      "Link your settlement flows or on-chain revenue sources via API or treasury wallets.",
    icon: (
      <svg
        width="32"
        height="32"
        viewBox="0 0 32 32"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
      >
        <rect
          x="4"
          y="8"
          width="24"
          height="16"
          rx="2"
          stroke="currentColor"
          strokeWidth="2"
        />
        <path d="M16 14V18M14 16H18" stroke="currentColor" strokeWidth="2" />
      </svg>
    ),
  },
  {
    id: 2,
    step: "STEP 2",
    title: "Set your collateral ratio",
    description:
      "Choose how much revenue (e.g. 30%) you lock into Biyard's reserve engine.",
    icon: (
      <svg
        width="32"
        height="32"
        viewBox="0 0 32 32"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
      >
        <rect
          x="6"
          y="6"
          width="20"
          height="20"
          rx="2"
          stroke="currentColor"
          strokeWidth="2"
        />
        <path
          d="M6 12H26M12 6V12M20 6V12"
          stroke="currentColor"
          strokeWidth="2"
        />
      </svg>
    ),
  },
  {
    id: 3,
    step: "STEP 3",
    title: "Launch on Biyard",
    description:
      "Deploy a revenue-backed token with liquidity rules and safety rails pre-configured.",
    icon: (
      <svg
        width="32"
        height="32"
        viewBox="0 0 32 32"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
      >
        <path
          d="M16 6L6 11L16 16L26 11L16 6Z"
          stroke="currentColor"
          strokeWidth="2"
          strokeLinejoin="round"
        />
        <path
          d="M6 21L16 26L26 21M6 16L16 21L26 16"
          stroke="currentColor"
          strokeWidth="2"
          strokeLinejoin="round"
        />
      </svg>
    ),
  },
  {
    id: 4,
    step: "STEP 4",
    title: "Monitor with DRE",
    description:
      "Use the DRE(Dynamic Reserve Engine) dashboard to track reserves, governance decisions and launch performance in real time.",
    icon: (
      <svg
        width="32"
        height="32"
        viewBox="0 0 32 32"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
      >
        <circle cx="16" cy="16" r="10" stroke="currentColor" strokeWidth="2" />
        <path
          d="M16 8V16L20 20"
          stroke="currentColor"
          strokeWidth="2"
          strokeLinecap="round"
        />
      </svg>
    ),
  },
];

export function HowItWorksSection() {
  return (
    <Section
      id="how-it-works"
      containerClassName="flex-col items-center gap-48"
    >
      <div className="flex flex-col items-center gap-16 text-center max-w-4xl">
        <p className="text-primary text-sm font-semibold uppercase tracking-wider">
          HOW IT WORKS
        </p>
        <h2 className="font-black text-[36px]/[48px] max-tablet:text-[28px]/[38px]">
          From your revenue to a live,{" "}
          <span className="text-primary">collateral-backed token</span>
        </h2>
      </div>

      <div className="grid grid-cols-2 gap-24 w-full max-w-800 max-tablet:grid-cols-1">
        {steps.map((step) => (
          <StepCard
            key={step.id}
            step={step.step}
            title={step.title}
            description={step.description}
            icon={step.icon}
          />
        ))}
      </div>
    </Section>
  );
}

function StepCard({
  step,
  title,
  description,
  icon,
}: {
  step: string;
  title: string;
  description: string;
  icon: React.ReactNode;
}) {
  return (
    <div className="flex flex-col p-32 border border-white/15 rounded-lg hover:border-primary/30 transition-colors">
      <div className="flex flex-col gap-12">
        <p className="text-primary text-xs font-semibold uppercase border border-primary inline-block w-fit px-12 py-4 rounded-full">
          {step}
        </p>
        <h3 className="text-white text-xl font-bold font-outfit">{title}</h3>
      </div>
      <p className="text-gray-400 text-sm font-light font-outfit pt-20">
        {description}
      </p>
      <div className="flex justify-end text-primary opacity-60 mt-auto">
        {icon}
      </div>
    </div>
  );
}
