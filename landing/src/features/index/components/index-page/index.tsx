import { HeroSection } from "../hero-section";
import { VisionMissionSection } from "../vision-mission-section";
import { WhatWeDoSection } from "../what-we-do-section";
import { TeamSection } from "../team-section";
import { ContactSection } from "../contact-section";
import { FooterSection } from "../footer-section";
import { Navigation } from "../hero-section/navigation";
import { Updates } from "../footer-section/updates";

export function IndexPage() {
  return (
    <div className="overflow-hidden">
      <Navigation />
      <HeroSection />
      <VisionMissionSection />
      <WhatWeDoSection />
      <TeamSection />
      <ContactSection />
      <Updates />
      <FooterSection />
    </div>
  );
}
