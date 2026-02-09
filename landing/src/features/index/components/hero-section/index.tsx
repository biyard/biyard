import { Button } from "@/components/button";
import { Section } from "@/components/section";
import {
  SectionLabel,
  PageTitle,
  BodyText,
  Highlight,
} from "@/components/typography";
import { scrollToSection, SectionIds } from "@/lib/utils";

export function HeroSection() {
  return (
    <Section
      id={SectionIds.Top}
      containerClassName="flex-row justify-between items-center max-tablet:flex-col"
    >
      <div className="absolute opacity-50 w-849 h-849 -top-356.74 -left-90 bg-purple-blur/80 blur-[500px] -z-1"></div>

      <div className="flex-1 flex flex-col items-start gap-44 max-tablet:items-center max-tablet:text-center">
        <div className="flex flex-col items-start gap-24 max-tablet:items-center">
          <SectionLabel>ENTERPRISE WEB3 LAUNCHPAD</SectionLabel>
          <PageTitle variant="hero" align="left" className="max-tablet:text-center">
            DON'T JUST
            <br />
            LAUNCH.
            <br />
            <Highlight>LAUNCH BACKED.</Highlight>
          </PageTitle>
          <BodyText size="large" className="max-w-500 max-tablet:text-center">
            A Web3 launchpad where tokens are grounded in real revenue and
            protected by stablecoin collateral. No hype—only trust. Designed for
            brands, fintechs, and Web3 innovators who demand a successful
            launch.
          </BodyText>
        </div>
        <div className="flex flex-row gap-16 items-center max-tablet:w-full">
            <Button variant="primary" size="medium" shape="rounded" onClick={() => {
              scrollToSection(SectionIds.Contact);
            }}>
              Talk to Our Team
            </Button>
          <Button variant="outline" size="medium" shape="rounded">
            Download Overview
          </Button>
        </div>
      </div>

      <div className="flex-1 relative flex justify-center items-center max-tablet:w-50% max-mobile:w-full aspect-square">
        <img
          src="/images/hero-cubes.svg"
          alt="3D Cubes Illustration"
          className="w-full h-full object-contain"
        />
      </div>
    </Section>
  );
}
