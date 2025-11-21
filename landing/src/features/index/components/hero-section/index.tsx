import { DotLottieReact } from "@lottiefiles/dotlottie-react";
import logo from "/logos/logo.json?url";
export function HeroSection() {
  return (
    <section
      id="top"
      className="overflow-hidden relative w-full"
      data-node-hydration="59"
    >
      <div className="absolute opacity-50 w-849 h-849 -top-356.74 -left-90 bg-purple-blur/80 blur-[500px] z-1"></div>

      <div className="flex flex-col justify-center w-full min-h-screen max-w-wrapper z-2 py-100 max-tablet:max-w-full">
        <div className="flex flex-col gap-64 items-center w-full max-tablet:gap-48">
          <div className="flex flex-col gap-32 items-center w-full max-w-648 max-tablet:gap-24">
            <DotLottieReact className="w-208" autoplay src={logo} speed={1} />
            <h1 className="font-black text-center uppercase text-[64px]/89 tracking-[-0.69px] max-tablet:text-[44px]/65 max-tablet:w-full">
              DEEP <span className="text-primary">TECH.</span>
              <br />
              DEEP <span className="text-primary">IMPACT.</span>
            </h1>
          </div>
        </div>
      </div>
    </section>
  );
}
