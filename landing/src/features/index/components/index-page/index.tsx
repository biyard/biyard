import { HeroSection } from "../hero-section";
import { VisionMissionSection } from "../vision-mission-section";
import { WhatWeDoSection } from "../what-we-do-section";
import { TeamSection } from "../team-section";
import { PressSection } from "../press-section";
import { ContactSection } from "../contact-section";
import { FooterSection } from "../footer-section";
import { Navigation } from "../hero-section/navigation";

export function IndexPage() {
  return (
    <div className="min-h-screen bg-slate-950">
      <Navigation />
      <HeroSection />
      <VisionMissionSection />
      <WhatWeDoSection />
      <TeamSection />
      <PressSection />
      <ContactSection />
      <FooterSection />
    </div>
  );
}
