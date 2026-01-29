import { Section } from "@/components/section";

export function HeroSection() {
  return (
    <Section
      id="top"
      containerClassName="flex-row justify-between items-center max-tablet:flex-col"
    >
      <div className="absolute opacity-50 w-849 h-849 -top-356.74 -left-90 bg-purple-blur/80 blur-[500px] z-1"></div>

      <div className="flex-1 flex flex-col items-start gap-44 max-tablet:items-center max-tablet:text-center">
        <div className="flex flex-col items-start gap-24 max-tablet:items-center">
          <p className="text-primary text-sm font-semibold tracking-[2px] uppercase">
            ENTERPRISE WEB3 LAUNCHPAD
          </p>
          <h1 className="font-black text-left uppercase text-[64px]/[72px] tracking-[-0.69px] max-tablet:text-[44px]/[52px] max-tablet:text-center">
            DON'T JUST
            <br />
            LAUNCH.
            <br />
            <span className="text-primary">LAUNCH BACKED.</span>
          </h1>
          <p className="text-gray-400 text-[16px]/[24px] font-light font-outfit leading-24 tracking-[0.25px] max-w-500 max-tablet:text-center">
            A Web3 launchpad where tokens are grounded in real revenue and
            protected by stablecoin collateral. No hype—only trust. Designed for
            brands, fintechs, and Web3 innovators who demand a successful
            launch.
          </p>
        </div>
        <div className="flex flex-row gap-16 items-center max-tablet:w-full">
          <button className="flex flex-row justify-center items-center px-40 py-12 gap-10 font-semibold rounded-xl text-black hover:opacity-90 transition-opacity bg-[linear-gradient(93.06deg,#00D190_0%,#A9B5F3_99.39%)] shadow-[0px_20px_50px_20px_rgba(0,230,165,0.15)] whitespace-nowrap">
            Talk to Our Team
          </button>
          <button className="flex flex-row justify-center items-center px-40 py-12 gap-10 font-semibold rounded-xl text-white hover:bg-white/10 transition-colors border border-white/20 whitespace-nowrap">
            Download Overview
          </button>
        </div>
      </div>

      <div className="flex-1 relative flex justify-center items-center max-tablet:w-full aspect-square">
        <img
          src="/images/hero-cubes.svg"
          alt="3D Cubes Illustration"
          className="w-full h-full object-contain"
        />
      </div>
    </Section>
  );
}
