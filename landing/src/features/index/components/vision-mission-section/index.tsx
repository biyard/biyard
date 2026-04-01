import {
  Inclusivity,
  Innovation,
  Integrity,
  LogoSymbol,
  Sustainability,
} from "@/components/icons";
import { useIntroI18n } from "./i18n";
import { MeetBiyard } from "@/components/icons";
import { Section } from "@/components/section";
import "./meet-biyard-animation.js";

export function VisionMissionSection() {
  const t = useIntroI18n();

  return (
    <Section id="intro" containerClassName="gap-96">
      <div className="flex flex-row justify-between items-start py-40 w-full border border-gray-800 backdrop-blur-sm rounded-[16px] px-118 gap-120 max-desktop:flex-col max-desktop:gap-40 max-desktop:px-20 max-tablet:flex-col max-tablet:py-24 max-tablet:px-16">
        <LabeledTextWithLogo
          icon={<LogoSymbol />}
          title={t.vision.title}
          description={t.vision.description}
        />

        <LabeledTextWithLogo
          icon={<LogoSymbol />}
          title={t.mission.title}
          description={t.mission.description}
        />
      </div>

      <div className="flex flex-row justify-between items-center w-full gap-50 max-tablet:flex-col">
        <MeetBiyard />
        <div className="flex flex-col gap-48 items-start max-tablet:w-full max-tablet:items-center">
          <h2 className="font-medium text-[45px]/64 max-tablet:w-full max-tablet:text-center">
            Core <span className="text-primary">Values</span>
          </h2>
          <div className="grid grid-cols-2 gap-y-80 w-full gap-x-51 max-mobile:grid-cols-1 max-tablet:gap-y-32 max-tablet:w-fit">
            <LabeledTextWithLogo
              icon={<Innovation />}
              title={t.innovation.title}
              description={t.innovation.description}
            />
            <LabeledTextWithLogo
              icon={<Inclusivity />}
              title={t.inclusivity.title}
              description={t.inclusivity.description}
            />
            <LabeledTextWithLogo
              icon={<Integrity />}
              title={t.integrity.title}
              description={t.integrity.description}
            />
            <LabeledTextWithLogo
              icon={<Sustainability />}
              title={t.sustainability.title}
              description={t.sustainability.description}
            />
          </div>
        </div>
      </div>
    </Section>
  );
}

function LabeledTextWithLogo({
  icon,
  title,
  description,
  smallGap = false,
}: {
  icon: React.ReactNode;
  title: string;
  description: string;
  smallGap?: boolean;
}) {
  return (
    <>
      <div className="flex flex-col gap-24">
        <div className="flex flex-row gap-12">
          <div
            className="flex flex-row justify-start items-center w-54 aria-sm:w-44"
            aria-sm={smallGap}
          >
            {icon}
          </div>
          <label className="font-semibold text-[28px]/36 tracking-[0px]">
            {title}
          </label>
        </div>
        <p
          className="font-extralight text-gray-300 whitespace-pre-line text-[15px]/23 tracking-[0px] ml-54 aria-sm:ml-44"
          aria-sm={smallGap}
        >
          {description}
        </p>
      </div>
    </>
  );
}
