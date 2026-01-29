import { HeroSection } from "../hero-section";
import { AboutUsSection } from "../about-us-section";
import { HowItWorksSection } from "../how-it-works-section";
import { SolutionSection } from "../solution-section";
import { PlatformsSection } from "../platforms-section";
import { FooterSection } from "../footer-section";
import { Navigation } from "../hero-section/navigation";
import { ContactSection } from "../contact-section";

export function IndexPage() {
  return (
    <div className="overflow-hidden pt-40">
      <Navigation />
      <HeroSection />
      <AboutUsSection />
      <SolutionSection />
      <PlatformsSection />
      <HowItWorksSection />
      <ContactSection />
      <FooterSection />
    </div>
  );
}
