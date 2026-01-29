import { Section } from "@/components/section";

const features = [
  {
    id: 1,
    title: "Revenue-backed collateral",
    text: "Lock a meaningful share of revenue as on-chain collateral so users know your token is anchored to real business performance.",
  },
  {
    id: 2,
    title: "Instant market trust",
    text: "Demonstrate transparent, auditable reserves from day one and differentiate from speculative launches.",
  },
  {
    id: 3,
    title: "No hype overhead",
    text: "We provide smart contracts, liquidity orchestration, monitoring and on-chain reporting out of the box.",
  },
  {
    id: 4,
    title: "No ICO, No STO, Regulation-ready",
    text: "Collateralized architecture built with financial-grade transparency to support institutional conversations.",
  },
  {
    id: 5,
    title: "Full on-chain audit trail",
    text: "Every collateral event, withdrawal and governance decision is permanently recorded on-chain.",
  },
  {
    id: 6,
    title: "DeFi native, enterprise friendly",
    text: "Bridge Web2 businesses into Web3 without compromising on security, accountability or UX.",
  },
];

export function AboutUsSection() {
  return (
    <Section
      id="about-us"
      // FIXME: use color palette
      className="bg-[#1A1D30]"
      containerClassName="flex-row justify-between items-start max-tablet:flex-col gap-36"
    >
      <div className="flex-1 flex flex-col items-start gap-8 max-tablet:w-full">
        <p className="text-primary text-sm font-semibold uppercase">ABOUT US</p>
        <h2 className="font-black text-left text-[30px]/[42px]">
          <p>Built for companies</p>
          <span className="text-primary">that need more than hype</span>
        </h2>
        <p className="text-gray-400 text-[14px]/[20px] font-light font-outfit leading-24 tracking-[0.25px]">
          Biyard turns your existing business revenue into a verifiable,
          on-chain reserve — giving every launch an instant trust layer, without
          forcing your team to become crypto infrastructure experts.
        </p>
        <img
          className="opacity-40 w-200 aspect-square mt-20"
          src="/images/aboutus_icon.svg"
          alt=""
        />
      </div>

      <div className="flex-2 grid grid-cols-2 gap-24 max-tablet:grid-cols-1">
        {features.map((feature) => (
          <FeatureCard
            key={feature.id}
            title={feature.title}
            text={feature.text}
          />
        ))}
      </div>
    </Section>
  );
}

function FeatureCard({ title, text }: { title: string; text: string }) {
  return (
    <div className="flex flex-col gap-12 items-start p-24">
      <div className="flex flex-row gap-8 justify-center items-center">
        <div className="w-6 h-6 rotate-45 bg-primary"></div>
        <h3 className="text-white text-base font-semibold font-outfit leading-24 tracking-[0.25px]">
          {title}
        </h3>
      </div>
      <p className="text-gray-400 text-sm font-light font-outfit leading-20 tracking-[0.25px]">
        {text}
      </p>
    </div>
  );
}
