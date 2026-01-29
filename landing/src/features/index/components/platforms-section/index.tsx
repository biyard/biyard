import { Section } from "@/components/section";
import { useState } from "react";
import { ChevronLeft, ChevronRight } from "lucide-react";

const platforms = [
  {
    id: 1,
    name: "Rotel",
    subtitle: "Decentralized Governance & Identity Platform",
    description:
      '"Rotel is a Web3 governance and identity platform that enables user credential, enhances community decision-making, and rewards meaningful participation."',
    logo: "/images/platforms/ratel.png",
  },
  {
    id: 2,
    name: "Voice Korea",
    subtitle: "Online Deliberation Platform",
    description:
      '"Voice Korea is a DID-based online deliberation platform that enables large-scale, tamper-proof public consultation."',
    logo: "/images/platforms/voice.png",
  },
  {
    id: 3,
    name: "SGU",
    subtitle: "",
    description:
      '"Biyard delivers a trusted identity and governance layer that makes large-scale and transparent on-chain governance model we expect to see widely adopted."',
    logo: "/images/platforms/sgu.png",
  },
];

export function PlatformsSection() {
  const [currentIndex, setCurrentIndex] = useState(0);

  const handlePrevious = () => {
    setCurrentIndex((prev) => (prev === 0 ? platforms.length - 1 : prev - 1));
  };

  const handleNext = () => {
    setCurrentIndex((prev) => (prev === platforms.length - 1 ? 0 : prev + 1));
  };

  return (
    <Section
      id="platforms"
      // FIXME: use color palette
      className="bg-[#1A1D30]"
      containerClassName="flex-col items-center gap-48 max-tablet:gap-32"
    >
      <div className="flex flex-col items-center gap-16 text-center max-w-800">
        <p className="text-primary text-sm font-semibold uppercase tracking-wider">
          CASE STUDY & TESTIMONIAL
        </p>
        <h2 className="font-bold text-[30px]/[42px] max-tablet:text-[24px]/[32px]">
          Platforms building <span className="text-primary">on Biyard</span>
        </h2>
        <p className="text-gray-400 text-[14px]/[20px] font-light font-outfit tracking-[0.25px]">
          From Web3-native protocols to Web2 payment companies, Biyard's R-DRE
          engine powers launches where revenue, reserves and governance stay
          tightly aligned.
        </p>
      </div>

      {/* Carousel */}
      <div className="relative w-full max-w-1200">
        <div className="flex items-center justify-between gap-32 max-tablet:flex-col">
          <button
            onClick={handlePrevious}
            className="size-48 flex items-center justify-center rounded-xl border border-white/25 max-tablet:hidden"
            aria-label="Previous platform"
          >
            <ChevronLeft className="text-white" size={24} />
          </button>

          {/* Cards Container */}
          <div className="flex-1 grid grid-cols-3 gap-24 max-tablet:grid-cols-1">
            {platforms.map((platform, index) => (
              <PlatformCard key={platform.id} platform={platform} />
            ))}
          </div>

          <button
            onClick={handleNext}
            className="size-48 flex items-center justify-center rounded-xl border border-white/25 max-tablet:hidden"
            aria-label="Next platform"
          >
            <ChevronRight className="text-white" size={24} />
          </button>
        </div>
      </div>
    </Section>
  );
}

interface PlatformCardProps {
  platform: {
    name: string;
    subtitle: string;
    description: string;
    logo: string;
  };
  className?: string;
}

function PlatformCard({ platform, className = "" }: PlatformCardProps) {
  return (
    <div
      className={`flex flex-col gap-20 items-start p-32 rounded-2xl border border-white/10 bg-[#1E2139] backdrop-blur-sm transition-all min-h-320 ${className}`}
    >
      <div className="size-64 flex items-center justify-center rounded-2xl bg-white/5 p-12">
        <img
          src={platform.logo}
          alt={`${platform.name} logo`}
          className="w-full h-full object-contain"
        />
      </div>

      <div className="flex flex-col gap-8">
        <h3 className="text-white text-xl font-semibold font-outfit leading-28 tracking-[0.25px]">
          {platform.name}
          {platform.subtitle && ": "}
          {platform.subtitle}
        </h3>
        <p className="text-gray-400 text-sm font-light font-outfit leading-20 tracking-[0.25px]">
          {platform.description}
        </p>
      </div>
    </div>
  );
}
