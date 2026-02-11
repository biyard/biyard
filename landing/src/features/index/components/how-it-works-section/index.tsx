import {
  Dashboard3,
  FlightTakeoff,
  LaptopSettings,
  Wallet,
} from "@/assets/icons";
import { Section } from "@/components/section";
import {
  SectionLabel,
  SectionTitle,
  CardTitle,
  BodyText,
  Highlight,
} from "@/components/typography";
import { SectionIds } from "@/lib/utils";

const steps = [
  {
    id: 1,
    step: "STEP 1",
    title: "Connect your revenue",
    description:
      "Link your settlement flows or on-chain revenue sources via API or treasury wallets.",
    icon: <Wallet />,
  },
  {
    id: 2,
    step: "STEP 2",
    title: "Set your collateral ratio",
    description:
      "Choose how much revenue (e.g. 30%) you lock into Biyard's reserve engine.",
    icon: <LaptopSettings />,
  },
  {
    id: 3,
    step: "STEP 3",
    title: "Launch on Biyard",
    description:
      "Deploy a revenue-backed token with liquidity rules and safety rails pre-configured.",
    icon: <FlightTakeoff />,
  },
  {
    id: 4,
    step: "STEP 4",
    title: "Monitor with DRE",
    description:
      "Use the DRE(Dynamic Reserve Engine) dashboard to track reserves, governance decisions and launch performance in real time.",
    icon: <Dashboard3 />,
  },
];

export function HowItWorksSection() {
  return (
    <Section
      id={SectionIds.HowItWorks}
      containerClassName="flex-col items-center gap-48"
    >
      <div className="flex flex-col items-center gap-16 text-center max-w-4xl">
        <SectionLabel>HOW IT WORKS</SectionLabel>
        <SectionTitle size="small" weight="bold" align="center">
          From your revenue to a live,{" "}
          <Highlight>collateral-backed token</Highlight>
        </SectionTitle>
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
    <div
      className="flex flex-col gap-16 items-start p-32
      bg-white/3 backdrop-blur-sm
      rounded-2xl border border-white/10
      bg-linear-to-br from-transparent to-white/4"
    >
      <div className="flex flex-col gap-12 backdrop-filter">
        <p className="text-primary text-xs font-semibold uppercase border border-primary inline-block w-fit px-12 py-4 rounded-full">
          {step}
        </p>
        <CardTitle size="medium" weight="bold">
          {title}
        </CardTitle>
      </div>
      <BodyText size="small" className="pt-20">
        {description}
      </BodyText>
      <div className="flex justify-end text-primary mt-auto [&>svg]:size-44">
        {icon}
      </div>
    </div>
  );
}

// absolute inset-0 bg-gradient-to-br from-amber-500/10 via-transparent to-transparent opacity-30 group-hover:opacity-50 transition-opacity duration-500
