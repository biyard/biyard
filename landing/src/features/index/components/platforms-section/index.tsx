import { Section } from "@/components/section";
import {
  SectionLabel,
  SectionTitle,
  CardTitle,
  BodyText,
  Highlight,
} from "@/components/typography";
import { SectionIds } from "@/lib/utils";
import { Swiper, SwiperSlide } from 'swiper/react';
import { Navigation } from 'swiper/modules';
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
  const showNavigation = platforms.length > 3;

  return (
    <Section
      id={SectionIds.Platforms}
      // FIXME: use color palette
      className="bg-[#1A1D30] platforms-section"
      containerClassName="flex-col items-center gap-48 max-tablet:gap-32"
    >
      <div className="flex flex-col items-center gap-16 text-center max-w-800">
        <SectionLabel>CASE STUDY & TESTIMONIAL</SectionLabel>
        <SectionTitle size="small" weight="bold" align="center">
          Platforms building <Highlight>on Biyard</Highlight>
        </SectionTitle>
        <BodyText size="medium">
          From Web3-native protocols to Web2 payment companies, Biyard's R-DRE
          engine powers launches where revenue, reserves and governance stay
          tightly aligned.
        </BodyText>
      </div>

      <div className="relative w-full max-w-1200">
        <Swiper
          modules={[Navigation]}
          navigation={showNavigation ? {
            prevEl: '.platforms-swiper-button-prev',
            nextEl: '.platforms-swiper-button-next',
          } : false}
          spaceBetween={24}
          slidesPerView={1}
          loop={platforms.length > 3}
          breakpoints={{
            0: {
              slidesPerView: 1,
              allowTouchMove: true,
            },
            768: {
              slidesPerView: 2,
              allowTouchMove: true,
            },
            1024: {
              slidesPerView: 3,
              allowTouchMove: false, 
            },
          }}
        >
          {platforms.map((platform) => (
            <SwiperSlide key={platform.id}>
              <PlatformCard platform={platform} />
            </SwiperSlide>
          ))}
        </Swiper>

        {showNavigation && (
          <>
            <button
              className="platforms-swiper-button-prev absolute -left-80 top-1/2 -translate-y-1/2 z-10 size-48 flex items-center justify-center rounded-xl border border-white/25 hover:bg-white/5 transition-colors max-tablet:hidden disabled:opacity-50 disabled:cursor-not-allowed"
              aria-label="Previous platform"
            >
              <ChevronLeft />
            </button>
            <button
              className="platforms-swiper-button-next absolute -right-80 top-1/2 -translate-y-1/2 z-10 size-48 flex items-center justify-center rounded-xl border border-white/25 hover:bg-white/5 transition-colors max-tablet:hidden disabled:opacity-50 disabled:cursor-not-allowed"
              aria-label="Next platform"
            >
              <ChevronRight />
            </button>
          </>
        )}
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
        <CardTitle size="large">
          {platform.name}
          {platform.subtitle && ": "}
          {platform.subtitle}
        </CardTitle>
        <BodyText size="small">{platform.description}</BodyText>
      </div>
    </div>
  );
}
